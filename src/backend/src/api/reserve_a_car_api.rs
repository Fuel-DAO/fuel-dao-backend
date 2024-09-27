
use candid::Principal;
use ic_cdk_macros::update;

use crate::{CarStatus, PaymentStatus, RentalTransaction, STATE};
#[update]
fn reserve_car(car_id: Principal, customer_id: Principal, start_timestamp: u64, end_timestamp: u64) -> Result<RentalTransaction, String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(car) = state.cars.get_mut(&car_id) {
            match car.status {
                CarStatus::Available => {
                    let reservation_id = Principal::anonymous(); // Generate a reservation ID
                    car.status = CarStatus::Reserved {
                        reservation_id,
                        reservation_timestamp: 0  as u64, // Current time as u64
                        customer_id,
                    };
                    let total_days = (end_timestamp - start_timestamp) / 86400; // Calculate total days
                    let total_amount = car.price_per_day * total_days as f64;    // Calculate total rental amount
                    let transaction = RentalTransaction {
                        car_principal_id: car_id,
                        customer_principal_id: customer_id,
                        customer_name: "Customer Name".to_string(), // Modify accordingly
                        start_timestamp,
                        end_timestamp,
                        total_amount,
                        payment_status: PaymentStatus::Unpaid,
                        reservation_id: Some(reservation_id),
                    };
                    return Ok(transaction);
                },
                _ => return Err("Car is not available".to_string()),
            }
        }
        Err("Car not found".to_string())
    })
}
