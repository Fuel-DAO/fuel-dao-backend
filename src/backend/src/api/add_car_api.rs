use ic_cdk_macros::update;

use crate::{models::CarDetails, STATE};


#[update]
fn add_car(car: CarDetails) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
         let id = state.cars.last_key_value().map_or(1, |f| f.0 + 1);
        state.cars.insert(id, crate::Car { id, details: CarDetails { id, ..car }, bookings: Vec::new(), /* monitoring: Vec::new()  */});
    });
}

#[update]
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
