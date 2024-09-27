use candid::Principal;
use ic_cdk_macros::query;

use crate::{models::CarDetails, STATE};
#[query]
fn get_car_details(car_id: Principal) -> Option<CarDetails> {
    STATE.with(|state| {
        let state = state.borrow();
        state.cars.get(&car_id).cloned() // Clone to return a copy
    })
}
