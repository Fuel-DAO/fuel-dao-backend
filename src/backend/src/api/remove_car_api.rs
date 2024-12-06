use ic_cdk_macros::update;

use crate::STATE;
use crate::{is_controller, Car};


#[update(guard = "is_controller")]
fn remove_car(id: u64) -> Option<Car> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.cars.remove(&id)
    })
}