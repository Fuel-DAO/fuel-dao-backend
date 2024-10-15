
use candid::Principal;
use ic_cdk::caller;
use ic_cdk::{api::time, query};
use ic_cdk_macros::update;

use crate::{ Car, CarStatus, CustomerDetials, PaymentStatus, RentalTransaction, TransactionHistory, STATE};
use crate::controller::is_controller;
use super::monitoring::log_car_checkout;
use super::send_email::{refresh_token, send_email_gmail};
#[update]
async fn reserve_car(car_id: u64, start_timestamp: u64, end_timestamp: u64,customer :CustomerDetials) -> Result<RentalTransaction, String> {
    if start_timestamp >= end_timestamp || start_timestamp < (time() / 1_000_000_000 ){
        return Err("Invalid time range".to_string());
    }

    customer.validate_details()?;

    let transaction =  STATE.with(|state| {
        let mut state = state.borrow_mut();

        match state.cars.get_mut(&car_id)  {
            Some(car) => {match  car_availibility( car.clone(), start_timestamp, end_timestamp) {
                Ok( mut transaction) => {
                    transaction.customer = Some(customer);
                    car.bookings.push(transaction.clone());
                     Ok(transaction)
                },
                _ =>  Err("Car is not available".to_string()),
            }},
            None => Err("Car not found".to_string())
        }
    });

    

    if transaction.is_ok() {

        log_car_checkout(car_id, transaction.as_ref().unwrap().booking_id);

        let mail_status =  send_email_gmail(transaction.clone().unwrap()).await;

        match mail_status {
            Err(e) if e.contains("invalid_token") => {
                let _ = refresh_token().await;
                let _ = send_email_gmail(transaction.clone().unwrap()).await;
            }
            _ => {}
        }
    }

     transaction
}


pub fn car_availibility(car: Car, start_timestamp: u64, end_timestamp: u64) -> Result<RentalTransaction, String> {
    match car.get_booking_status_at_give_time_period(start_timestamp, end_timestamp) {
        CarStatus::Available => {
            let customer_id = ic_cdk::caller(); 
           
            let total_days = (end_timestamp - start_timestamp) as f64 / 86400.0; 
            let total_amount = car.details.current_price_per_day * total_days as f64;  
            let transaction = RentalTransaction {
                booking_id: time(),
                car_id: car.id,
                customer_principal_id: customer_id,
                start_timestamp,
                customer: None,
                end_timestamp,
                total_amount,
                payment_status: PaymentStatus::Unpaid,
            };
            return Ok(transaction);
        },
        _ => return Err("Car is not available".to_string()),
    }

}

#[query(guard="is_controller")]
pub fn all_bookings() -> Vec<Vec<RentalTransaction>> {
    STATE.with(|state| {
        state.borrow().cars.iter().map(|f| f.1.bookings.clone()).collect()
    })
}

#[query(guard="is_controller")]
pub fn user_bookings(user: Principal) -> Vec<RentalTransaction> {
    STATE.with(|state| {
        state.borrow().cars.iter().map(|f| f.1.bookings.clone().iter().filter(|f| f.customer_principal_id == user).cloned().collect::<Vec<RentalTransaction>>()).flatten().collect()
    })
}

#[query]
pub fn booking_details(car_id: u64, booking_id: u64) -> Option<TransactionHistory> {
    STATE.with(|state| {
        state.borrow().cars.iter().find(|f| *f.0 == car_id).map(|f| f.1.bookings.iter().find(|f| f.booking_id == booking_id).cloned()).flatten().map(|f| f.to_transaction_history())
    })
}

#[query]
pub fn current_user_bookings() -> Vec<RentalTransaction> {
    let user = caller();
    STATE.with(|state| {
        state.borrow().cars.iter().map(|f| f.1.bookings.clone().iter().filter(|f| f.customer_principal_id == user).cloned().collect::<Vec<RentalTransaction>>()).flatten().collect()
    })
}