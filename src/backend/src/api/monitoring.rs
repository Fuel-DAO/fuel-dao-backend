
use std::collections::VecDeque;

 use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};
use crate::controller::is_controller;
use crate::STATE;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
 pub enum EventMoniter {
     SearchInitiate { current_timestamp: u64, user_principal: Principal}, 
     SelectedCar{car_id: u64 ,current_timestamp: u64, user_principal: Principal }, 
     CarCheckout{car_id: u64, current_timestamp: u64, user_principal: Principal},
 }

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct MonitoringState {
    events: VecDeque<EventMoniter>,  // Global event log for easier access
}

impl MonitoringState {
    /// Log the initiation of a car search
    pub fn log_search_initiated(&mut self, user: Principal) {
        let event = EventMoniter::SearchInitiate {
            current_timestamp: time() as u64,
            user_principal: user,
        };
        self.events.push_back(event);
    }

    /// Log the selection of a car
    pub fn log_car_selected(&mut self, user: Principal, car_id: u64) {
        let event = EventMoniter::SelectedCar {
            current_timestamp: time() as u64,
            user_principal: user,
            car_id,
        };
        self.events.push_back(event);
    }

    /// Log the car checkout
    pub fn log_car_checkout(&mut self, user: Principal, car_id: u64) {
        let event = EventMoniter::CarCheckout {
            current_timestamp: time() as u64,
            user_principal: user,
            car_id,
        };
        self.events.push_back(event);
    }

    /// Retrieve all monitoring events
    pub fn get_all_events(&self) -> Vec<EventMoniter> {
        self.events.iter().cloned().collect()
    }

    /// Retrieve events filtered by user
    pub fn get_events_by_user(&self, user: Principal) -> Vec<EventMoniter> {
        self.events
            .iter()
            .filter(|event| match event {
                EventMoniter::SearchInitiate { user_principal: u, .. } => u == &user,
                EventMoniter::SelectedCar { user_principal: u, .. } => u == &user,
                EventMoniter::CarCheckout { user_principal: u, .. } => u == &user,
            })
            .cloned()
            .collect()
    }
}

// #[ic_cdk_macros::update]
pub fn log_search() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let user = ic_cdk::caller();
        state.monitoring.log_search_initiated(user);
    });
}

// #[ic_cdk_macros::update]
pub fn log_car_selection(car_id: u64) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let user = ic_cdk::caller();
        state.monitoring.log_car_selected(user, car_id);
    });
}

pub fn log_car_checkout(car_id: u64) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let user = ic_cdk::caller();
        state.monitoring.log_car_checkout(user, car_id);
    });
}

#[ic_cdk_macros::query (guard = "is_controller") ]
fn get_monitoring_events() -> Vec<EventMoniter> {
    STATE.with(|state| state.borrow().monitoring.get_all_events())
}
