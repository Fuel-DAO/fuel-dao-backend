use ic_cdk_macros::query;

use crate::{ Car, STATE};
#[query]
fn get_car_details(car_id: u64) -> Option<Car> {
    STATE.with(|state| {
        let state = state.borrow();
        state.cars.get(&car_id).cloned() // Clone to return a copy
    })
}
