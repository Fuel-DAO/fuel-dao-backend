use ic_cdk_macros::query;

use crate::{  CarAvailability, STATE};

use super::reserve_a_car_api::car_availibility;
#[query]
fn get_car_details(car_id: u64, start_time: u64, end_time: u64) -> Option<CarAvailability> {
    STATE.with(|state| {
        let state = state.borrow();
        state.cars.get(&car_id).cloned().map(|mut f| {f.details.status = f.get_booking_status_at_give_time_period(start_time, end_time);  CarAvailability{
            details: f.details.clone(), 
            available: car_availibility(f.clone(), start_time, end_time).ok()
        }  }) 
    })
}
