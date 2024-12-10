use candid::Principal;
use ic_cdk_macros::*;
use super::admin::is_controller;
use crate::STATE;

#[update(guard = "is_controller")]
pub fn set_provision_canister(principal: Principal) -> Result<bool, String> {
    STATE.with(|state| {
        state.borrow_mut().provision_canister = Some(principal);
    });

    Ok(true)
}

/// Get the provision canister
#[query]
pub fn get_provision_canister() -> Option<Principal> {
    STATE.with(|state| state.borrow().provision_canister)
}

/// Set the temporary asset canister
#[update(guard = "is_controller")]
pub fn set_temp_asset_canister(principal: Principal) -> Result<bool, String> {
    STATE.with(|state| {
        state.borrow_mut().temp_asset_canister = Some(principal);
    });

    Ok(true)
}

/// Get the temporary asset canister
#[query]
pub fn get_temp_asset_canister() -> Option<Principal> {
    STATE.with(|state| state.borrow().temp_asset_canister)
}