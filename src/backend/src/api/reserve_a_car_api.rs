
use candid::Principal;
use ic_cdk::caller;
use ic_cdk::{api::time, query};
use ic_cdk_macros::update;

use crate::{Car, CarStatus, CustomerDetials, PaymentStatus, RentalTransaction, STATE};
use crate::controller::is_controller;
use super::monitoring::log_car_checkout;
#[update]
fn reserve_car(car_id: u64, start_timestamp: u64, end_timestamp: u64,customer :CustomerDetials) -> Result<RentalTransaction, String> {
    if start_timestamp >= end_timestamp || start_timestamp < (time() / 1_000_000_000 ){
        return Err("Invalid time range".to_string());
    }

    log_car_checkout(car_id);
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(car) = state.cars.get_mut(&car_id) {
            match  car_availibility( car.clone(), start_timestamp, end_timestamp) {
                Ok( mut transaction) => {
                    transaction.customer = Some(customer);
                    car.bookings.push(transaction.clone());
                    return Ok(transaction);
                },
                _ => return Err("Car is not available".to_string()),
            }
        }
        Err("Car not found".to_string())
    })
}


pub fn car_availibility(car: Car, start_timestamp: u64, end_timestamp: u64) -> Result<RentalTransaction, String> {
    match car.get_booking_status_at_give_time_period(start_timestamp, end_timestamp) {
        CarStatus::Available => {
            let customer_id = ic_cdk::caller(); 
           
            let total_days = (end_timestamp - start_timestamp) as f64 / 86400.0; 
            let total_amount = car.details.current_price_per_day * total_days as f64;  
            let transaction = RentalTransaction {
                car_principal_id: car.id,
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
pub fn current_user_bookings() -> Vec<RentalTransaction> {
    let user = caller();
    STATE.with(|state| {
        state.borrow().cars.iter().map(|f| f.1.bookings.clone().iter().filter(|f| f.customer_principal_id == user).cloned().collect::<Vec<RentalTransaction>>()).flatten().collect()
    })
}