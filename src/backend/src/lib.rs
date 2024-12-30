use std::{cell::RefCell, collections::BTreeMap};
pub mod models;
use api::monitoring::MonitoringState;
use life_cycle::memory::State;
pub use models::*;
mod api;
pub mod constant;
mod controller;
pub mod default;
pub use api::monitoring::EventMoniter;
pub use candid::Principal;
pub use controller::*;
mod life_cycle;
// use crate::api::send_email::MailState;
#[cfg(test)]
mod tests;
pub mod utils;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());

    static SNAPSHOT_DATA: RefCell<Vec<u8>> = RefCell::default();

}




#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}! from FuelEV", name)
}

ic_cdk_macros::export_candid!();
