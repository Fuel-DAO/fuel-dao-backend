use std::{cell::RefCell, collections::BTreeMap};
pub mod models;
use api::monitoring::MonitoringState;
use default::DEFAULT_CAR_ID;
use ic_cdk::{post_upgrade, pre_upgrade, storage};
pub use models::*;
pub mod default;
mod api;
mod controller;
pub mod constant;
pub use api::monitoring::EventMoniter;
pub use candid::Principal;
pub use controller::*;
#[cfg(test)]
mod tests;


thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        cars: BTreeMap::new(),
        monitoring: MonitoringState::default(),
        controllers: Vec::new(),
    });
}

#[ic_cdk_macros::init]
fn init() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Parse the hardcoded Principal for the default car
        // let default_car_principal = Principal::from_text(DEFAULT_CAR_PRINCIPAL).expect("Invalid principal format");
        
        // Create the default car
        let default_car = CarDetails::default();


        if !state.cars.contains_key(&DEFAULT_CAR_ID )  {
            state.cars.insert(DEFAULT_CAR_ID, Car { id: DEFAULT_CAR_ID, details: default_car, bookings: Vec::new(),/*  monitoring: Vec::new() */});
        }
        
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| storage::stable_save((State {
        cars: state.borrow().cars.clone(), 
        monitoring: state.borrow().monitoring.clone(),
        controllers: state.borrow().controllers.clone(),
    },)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let state: Result<(State, ), String> = storage::stable_restore();
    match state {
        Ok(state) => {
            STATE.with(|s| { *s.borrow_mut() =  state.0;  });

        }, Err(e) => {
            println!("Failed to do post upgrade {e}");
        }
    }
}



#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


ic_cdk_macros::export_candid!();
