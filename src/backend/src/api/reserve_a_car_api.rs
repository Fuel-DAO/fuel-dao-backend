use std::collections::BTreeMap;

use candid::Principal;
use ic_cdk::caller;
use ic_cdk::{api::time, query};
use ic_cdk_macros::update;

// use super::monitoring::log_car_checkout;
use crate::controller::is_controller;
use crate::{
    Car, CarStatus, CustomerDetials, PaymentStatus, RazorpayPayment, RentalTransaction, TransactionHistory, STATE
};

// use super::monitoring::log_car_checkout;
// use super::send_email::{refresh_token, send_email_gmail};

#[query] 
fn get_unpaid_booking_details_by_booking_id(booking_id: u64) -> Option<RentalTransaction> {
    STATE.with(|f| f.borrow().unpaid_bookings.get(&booking_id).cloned())
}  

#[update]
fn validate_details_and_availaibility(
    car_id: u64,
    start_timestamp: u64,
    end_timestamp: u64,
    customer: CustomerDetials,) -> Result<RentalTransaction, String> {

        if start_timestamp >= end_timestamp || start_timestamp < (time() / 1_000_000_000) {
            return Err("Invalid time range".to_string());
        }

        customer.validate_details()?;

       let rental_booking = match   STATE.with_borrow(|f| f.clone()).cars.get(&car_id).map(|car| car_availibility(car.clone(), start_timestamp, end_timestamp, customer.caller)) {
           Some(f) => f,
           None => return  Err("Invalid data".to_string()),
       };

       rental_booking.map(|mut f| {
        f.customer = Some(customer);
        f.save_to_unpaid_bookings(); 
        f
       })
}

#[update(guard = "is_controller")]
async fn reserve_car(
    // car_id: u64,
    // start_timestamp: u64,
    // end_timestamp: u64,
    // customer: CustomerDetials,
    booking_id: u64,
    payment: RazorpayPayment
) -> Result<RentalTransaction, String> {

    let mut booking = get_unpaid_booking_details_by_booking_id(booking_id).ok_or("Invalid booking id".to_string())?;

    let car_id = booking.car_id;

    booking.payment_status = PaymentStatus::Paid { payment };

    let transaction = STATE.with(|state| {
        let mut state = state.borrow_mut();
        // Get all the details from unpaid bookings based on booking_id
        match state.cars.get_mut(&car_id) {
            Some(car) => {
                car.bookings.insert(booking_id, booking.clone());
                state.monitoring.log_car_checkout(booking.customer_principal_id, car_id, booking_id);
                Ok(booking)
            },
            None => Err("Car not found".to_string()),
        }
    });
    transaction.map(|f| { f.remove_from_unpaid_bookings_by_booking_id() ; f})
}

pub fn car_availibility(
    car: Car,
    start_timestamp: u64,
    end_timestamp: u64,
    caller: Principal,
) -> Result<RentalTransaction, String> {
    match car.get_booking_status_at_give_time_period(start_timestamp, end_timestamp) {
        CarStatus::Available => {
            let customer_id = caller;

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
        }
        _ => return Err("Car is not available".to_string()),
    }
}

#[query(guard = "is_controller")]
pub fn all_bookings() -> Vec<BTreeMap<u64, RentalTransaction>> {
    STATE.with(|state| {
        state
            .borrow()
            .cars
            .iter()
            .map(|f| f.1.bookings.clone())
            .collect()
    })
}

#[query(guard = "is_controller")]
pub fn user_bookings(user: Principal) -> Vec<TransactionHistory> {
    STATE.with(|state| {
        state
            .borrow()
            .cars
            .iter()
            .map(|f| {
                f.1.bookings
                    .clone()
                    .iter()
                    .filter(|f| f.1.customer_principal_id == user)
                    .map(|f| f.1.clone().to_transaction_history())
                    .collect::<Vec<TransactionHistory>>()
            })
            .flatten()
            .collect()
    })
}

#[query]
pub fn booking_details(car_id: u64, booking_id: u64) -> Option<TransactionHistory> {
    STATE.with(|state| {
        state
            .borrow()
            .cars
            .iter()
            .find(|f| *f.0 == car_id)
            .map(|f| f.1.bookings.get(&booking_id))
            .flatten()
            .map(|f| f.to_transaction_history())
    })
}

#[query]
pub fn current_user_bookings() -> Vec<RentalTransaction> {
    let user = caller();
    STATE.with(|state| {
        state
            .borrow()
            .cars
            .iter()
            .map(|f| {
                f.1.bookings
                    .clone()
                    .iter()
                    .filter(|f| f.1.customer_principal_id == user)
                    .map(|f| f.1.clone())
                    .collect::<Vec<RentalTransaction>>()
            })
            .flatten()
            .collect()
    })
}

