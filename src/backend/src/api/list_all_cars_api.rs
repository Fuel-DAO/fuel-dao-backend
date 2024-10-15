use ic_cdk::update;

use crate::{Car, STATE};

use super::monitoring::log_search;
#[update ]
fn search_car(start_time: u64, end_time: u64) -> Vec<Car> {
    log_search();
    STATE.with(|state| {
        let state = state.borrow();
        state.cars.values().cloned().map(|mut f|{ f.details.status = f.get_booking_status_at_give_time_period(start_time, end_time); f.get_car_without_bookings()}).collect()
    })
}

#[ic_cdk_macros::update]
fn list_all_cars() -> Vec<Car> {
    log_search();
    STATE.with(|state| {
        let state = state.borrow();
        state.cars.values().map(|f| f.get_car_without_bookings()).collect()
    })
}


// #[query]
// fn list_all_cars(start_time: u64, end_time:u64) -> Vec<Car> {
//     STATE.with(|state| {
//         let state = state.borrow();
//         state.iter().map( status == UnderMaintainace ? Status::Unavailable value.reservation_status(date time) ).collect()
//   Car {status: Unaavilable/Availble}
//         state.cars.values().cloned().collect() // Return a list of all cars
//     })
// }
