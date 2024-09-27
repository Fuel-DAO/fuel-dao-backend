use ic_cdk_macros::query;

use crate::{CarDetails, STATE};
#[query]
fn list_all_cars() -> Vec<CarDetails> {
    STATE.with(|state| {
        let state = state.borrow();
        state.cars.values().cloned().collect() // Return a list of all cars
    })
}
