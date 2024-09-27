use std::{cell::RefCell, collections::BTreeMap};
use default::DEFAULT_CAR_PRINCIPAL;
use candid::Principal;
pub mod models;
pub use models::*;
pub mod utils;
pub mod default;
mod api;


thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        cars: BTreeMap::new(),
    });
}

#[ic_cdk_macros::init]
fn init() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Parse the hardcoded Principal for the default car
        let default_car_principal = Principal::from_text(DEFAULT_CAR_PRINCIPAL).expect("Invalid principal format");
        
        // Create the default car
        let default_car = CarDetails::default();
        
        // Insert the default car into the state
        state.cars.insert(default_car_principal, default_car);
    });
}



#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


ic_cdk_macros::export_candid!();
