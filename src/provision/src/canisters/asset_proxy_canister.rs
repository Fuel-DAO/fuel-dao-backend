use candid::Principal;
use crate::admin::admin::is_controller;
use crate::STATE;



#[ic_cdk_macros::update(guard= "is_controller")]
pub fn set_asset_proxy_canister(canister: Principal) -> Result<bool, String> {
    STATE.with(|state| {
        // let  proxy_canister = state.borrow_mut().asset_proxy_canister;
        // if proxy_canister.is_some() {
        //     return Err("Asset proxy canister already set".to_string());
        // }
        state.borrow_mut().asset_proxy_canister = Some(canister);
        Ok(true)
    })
}

#[ic_cdk_macros::query]
pub fn get_asset_proxy_canister() -> Option<Principal> {
    STATE.with(|state| {
        state.borrow().asset_proxy_canister
    })
}