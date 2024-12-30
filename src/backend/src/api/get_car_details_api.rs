use ic_cdk::{caller, update};

use crate::{  CarAvailability,  STATE};

use super::{monitoring::log_car_selection, reserve_a_car_api::car_availibility};
#[update]
fn get_car_details(car_id: u64, start_time: u64, end_time: u64) -> Option<CarAvailability> {
    log_car_selection(car_id);
    STATE.with(|state| {
        let state: std::cell::Ref<'_, crate::State> = state.borrow();
        state.cars.get(&car_id).map(|mut f| {f.details.status = f.get_booking_status_at_give_time_period(start_time, end_time);  CarAvailability{
            details: f.details.clone(), 
            available: car_availibility(f.clone(), start_time, end_time, caller()).ok()
        }  }) 
    })
}
