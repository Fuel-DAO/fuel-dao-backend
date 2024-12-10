use candid::Principal;

use crate::STATE;

pub fn is_controller() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if ic_cdk::api::is_controller(&caller) {
        return Ok(());
    }
    STATE.with(|state| {
        if state.borrow().admins.contains(&ic_cdk::caller()) {
            Ok(())
        } else {
            Err("You are not authorized to perform this action.".to_string())
        }
    })
}

pub fn is_provision_controller() -> Result<(), String> {
    let caller = ic_cdk::caller();
    STATE.with(|state| {
        if state.borrow().provision_canister == Some(caller) {
            Ok(())
        } else {
            Err("You are not authorized to perform this action.".to_string())
        }
    })
}

pub fn validate_asset_uploader() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        Ok(())
    } else {
        Err("You are not authorized to perform this action.".to_string())
    }
}
