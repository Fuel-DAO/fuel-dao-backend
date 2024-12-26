use ic_cdk_macros::{query, update};
use crate::is_controller;

use crate::{RentalTransaction, STATE};
use crate::models::CarTravelStats;


#[query]
pub fn car_stats() -> CarTravelStats {
    let state = STATE.with(|state| state.borrow().clone());
    let bookings: Vec<RentalTransaction> = state.cars.iter().map(|f| &f.1.bookings).flatten().map(|f| f.1.without_customer()).collect();
    let total_revenue: f64 = bookings.iter().map(|f| f.total_amount).sum();
    let total_distance_travelled: f64 = state.car_travel_details.iter().map(|f| f.1.distance).sum();
    CarTravelStats {
        total_revenue,
        total_distance_travelled,
        rentals: bookings,
        total_investment: 21_50_000_f64
    }
}

#[update(guard = "is_controller")]
pub fn add_distance_travelled(car_id: u64, distance: f64, notes: Option<String>) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let current_timestamp = ic_cdk::api::time();
        let caller = ic_cdk::caller();
        state.car_travel_details.insert(car_id, crate::DistanceTravelled { car_id, distance, current_timestamp, caller, notes });
    });
}