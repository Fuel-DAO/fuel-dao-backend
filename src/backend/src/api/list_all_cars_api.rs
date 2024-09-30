use ic_cdk_macros::query;

use crate::{Car,  STATE};
#[query]
fn list_all_cars() -> Vec<Car> {
    STATE.with(|state| {
        let state = state.borrow();
        state.cars.values().cloned().collect() // Return a list of all cars
    })
}


// #[query]
// fn list_all_cars(start_time: u128, end_time:u128) -> Vec<Car> {
//     STATE.with(|state| {
//         let state = state.borrow();
//         state.iter().map( status == UnderMaintainace ? Status::Unavailable value.reservation_status(date time) ).collect()
//   Car {status: Unaavilable/Availble}
//         state.cars.values().cloned().collect() // Return a list of all cars
//     })
// }
