use std::collections::BTreeMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::{collection::*, STATE};
use crate::admin::admin::is_controller;


#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct State {
    pub asset_wasm: Option<Vec<u8>>,
    pub token_wasm: Option<Vec<u8>>,
    pub admins: Vec<Principal>,
    pub collection_requests: BTreeMap<u64,CollectionRequestConfig>,
    pub asset_proxy_canister: Option<Principal>,

}

#[ic_cdk_macros::update (guard = "is_controller") ]
fn add_token_wasm(wasm: Vec<u8>) -> bool {
    STATE.with(|state| state.borrow_mut().token_wasm = Some(wasm) );
    true
}

#[ic_cdk_macros::update (guard = "is_controller") ]
fn include_wasm() -> bool {
    let wasm = include_bytes!("../../../wasm/token/token.wasm.gz").to_vec();
    STATE.with(|state| state.borrow_mut().token_wasm = Some(wasm) );
    true
}

#[ic_cdk_macros::query]
pub fn get_token_wasm() -> Option<Vec<u8>> {
    STATE.with(|state| state.borrow_mut().token_wasm.clone() )
}

#[ic_cdk_macros::update (guard = "is_controller") ]
fn add_asset_wasm(wasm: Vec<u8>) -> bool {
    STATE.with(|state| state.borrow_mut().asset_wasm = Some(wasm) );
    true
}


#[ic_cdk_macros::query]
pub fn get_asset_wasm() -> Option<Vec<u8>> {
    STATE.with(|state| state.borrow_mut().asset_wasm.clone() )
}