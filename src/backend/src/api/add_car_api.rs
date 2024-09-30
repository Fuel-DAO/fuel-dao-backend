use ic_cdk_macros::update;

use crate::{models::CarDetails, STATE};


#[update]
fn add_car(car: CarDetails) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
         let id = state.cars.last_key_value().map_or(1, |f| f.0 + 1);
        state.cars.insert(id, crate::Car { id: id, details: car });
    });
}

#[update]
fn update_car(id: u64, car: CarDetails) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.cars.insert(id, crate::Car { id: id, details: car });
    });
}
