use std::cell::RefCell;

use life_cycle::memory::State;

pub mod state;
pub mod admin;

pub mod collection;
pub use collection::*;


use candid::Principal;

pub mod canisters;
pub use canisters::*;

pub mod life_cycle;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(Default::default());
    static SNAPSHOT_DATA: RefCell<Vec<u8>> = RefCell::default();
}


#[ic_cdk_macros::init]
fn init() {
    init_hook();
}

fn init_hook() {
    // STATE.with(|state| {
    //     let mut state = state.borrow_mut();
    // });
}




ic_cdk_macros::export_candid!();
