use candid::Principal;
use ic_cdk_macros::update;

use crate::{models::CarDetails, STATE};


#[update]
fn add_car(car: CarDetails) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let car_id = Principal::anonymous(); // Replace with actual ID generation logic
        state.cars.insert(car_id, car);
    });
}
