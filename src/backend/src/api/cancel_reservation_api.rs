use ic_cdk_macros::update;
use crate::STATE;
use crate::is_controller;

#[update(guard = "is_controller")]
fn cancel_reservation(car_id: u64, booking_id: u64) -> Result<String, String> {
    // let customer_id = ic_cdk::caller();
    STATE.with_borrow_mut(|state| {
        let  get_car =state.cars.get(&car_id);
        if let Some(mut car) = get_car {
            let booking =  car.bookings.get(&booking_id);
            match booking {
                Some(_) => {
                    car.bookings.remove(&booking_id);
                    state.cars.insert(car_id,car.clone());
                    return Ok("Reservation cancelled".to_string());
                },
                None => return Err("No active reservation found for this car".to_string()),
            }
        }
        Err("Car not found".to_string())
    })
}
