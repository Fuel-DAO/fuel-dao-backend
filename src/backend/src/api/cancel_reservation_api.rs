use ic_cdk_macros::update;
use crate::STATE;
use crate::is_controller;

#[update(guard = "is_controller")]
fn cancel_reservation(car_id: u64, booking_id: u64) -> Result<String, String> {
    // let customer_id = ic_cdk::caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(car) = state.cars.get_mut(&car_id) {
            let booking =  car.bookings.remove(&booking_id);
            match booking {
                Some(_) =>  return Ok("Reservation cancelled".to_string()),
                None => return Err("No active reservation found for this car".to_string()),
            }
        }
        Err("Car not found".to_string())
    })
}
