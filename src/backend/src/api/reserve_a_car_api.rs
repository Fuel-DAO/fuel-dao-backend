
use ic_cdk_macros::update;

use crate::{Car, CarStatus, PaymentStatus, RentalTransaction, STATE};

use super::monitoring::log_car_checkout;
#[update]
fn reserve_car(car_id: u64, start_timestamp: u64, end_timestamp: u64) -> Result<RentalTransaction, String> {
    if start_timestamp >= end_timestamp {
        return Err("Invalid time range".to_string());
    }

    log_car_checkout(car_id);
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(car) = state.cars.get_mut(&car_id) {
            match  car_availibility( car.clone(), start_timestamp, end_timestamp) {
                Ok(transaction) => {
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
                customer_name: "Customer Name".to_string(),
                start_timestamp,
                end_timestamp,
                total_amount,
                payment_status: PaymentStatus::Unpaid,
            };
            return Ok(transaction);
        },
        _ => return Err("Car is not available".to_string()),
    }

}