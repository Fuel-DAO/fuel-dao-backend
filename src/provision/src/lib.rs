use std::cell::RefCell;

use ic_cdk::storage;
use state::State;

pub mod state;
pub mod admin;

pub mod collection;
pub use collection::*;


use candid::Principal;
use ic_cdk_macros::*;

pub mod canisters;
pub use canisters::*;



thread_local! {
    static STATE: RefCell<State> = RefCell::new(Default::default());
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

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| storage::stable_save((State {
        admins: state.borrow().admins.clone(), 
        collection_requests: state.borrow().collection_requests.clone(),
        asset_wasm: state.borrow().asset_wasm.clone(),
        token_wasm: state.borrow().token_wasm.clone(),
        asset_proxy_canister: state.borrow().asset_proxy_canister.clone(),
    },)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let state: Result<(State, ), String> = storage::stable_restore();
    match state {
        Ok(state) => {
            STATE.with(|s| { *s.borrow_mut() =  state.0;  });
            init_hook();
        }, Err(e) => {
            println!("Failed to do post upgrade {e}");
        }
    }
}

ic_cdk_macros::export_candid!();
