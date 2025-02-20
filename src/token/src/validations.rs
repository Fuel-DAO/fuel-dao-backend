use candid::Principal;
use ic_cdk::caller;

use crate::STATE;

pub fn check_collection_owner() -> Result<(), String> {
    STATE.with(|f| match &f.borrow().metadata {
        Some(m) if m.metadata.collection_owner == caller() => Ok(()),
        _ => Err("You are not authorized to perform this action.".to_string()),
    })
}

pub fn check_controller() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if ic_cdk::api::is_controller(&caller){ return  Ok(());} else {
         Err("You are not authorized to perform this action.".to_string())
    }
}

pub fn check_not_anonymous() -> Result<(), String> {
    if Principal::anonymous() == caller(){ return  Err("You are not authorized to perform this action.".to_string()) };
    Ok(())
}
