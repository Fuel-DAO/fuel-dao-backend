use ic_cdk_macros::update;
use crate::{CarStatus, STATE};
#[update]
fn cancel_reservation(car_id: u64,) -> Result<String, String> {
    let customer_id = ic_cdk::caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(car) = state.cars.get_mut(&car_id) {
            match car.details.status {
                CarStatus::Reserved { customer_id: res_customer_id, .. } if res_customer_id == customer_id => {
                    car.details.status = CarStatus::Available; // Mark car as available again
                    return Ok("Reservation cancelled".to_string());
                },
                _ => return Err("No active reservation found for this car".to_string()),
            }
        }
        Err("Car not found".to_string())
    })
}
