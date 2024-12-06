use std::collections::BTreeMap;

use ic_cdk_macros::update;

use crate::{models::CarDetails, STATE};
use crate::is_controller;


#[update(guard = "is_controller")]
fn add_car(car: CarDetails) -> u64 {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
         let id = state.cars.last_key_value().map_or(1, |f| f.0 + 1);
        state.cars.insert(id, crate::Car { id, details: CarDetails { id, ..car }, bookings: BTreeMap::new(), /* monitoring: Vec::new()  */});
        id
    })
}

#[update(guard = "is_controller")]
fn update_car(id: u64, car: CarDetails) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.cars.get_mut(&id).map(|f| {
            f.details = car;
            f
        } );
        // let bookings = state.cars.get(&id).map_or( Vec::new() ,|f| f.bookings.clone());
        // state.cars.insert(id, crate::Car { id: id, details: car, bookings });
    });
}
