use candid::Principal;

use crate::STATE;

pub fn is_controller() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if ic_cdk::api::is_controller(&caller){ return  Ok(());}
    STATE.with(|state| {
        if state
            .borrow()
            .admins
            .contains(&ic_cdk::caller())
        {
            Ok(())
        } else {
            Err("You are not authorized to perform this action.".to_string())
        }
    })
}


#[ic_cdk_macros::query (guard = "is_controller") ]
fn get_controllers() -> Vec<Principal> {
    STATE.with(|state| state.borrow().admins.clone())
}


#[ic_cdk_macros::query]
pub fn is_admin(principal: Option<Principal>) -> bool {
    let principal = if let Some(principal) =  principal {
        principal
    } else {
        ic_cdk::caller()
    };
    STATE.with(|f| f.borrow().admins.contains(&principal))

}


#[ic_cdk_macros::update (guard = "is_controller") ]
fn add_admin(principal: Principal) -> bool {
    STATE.with(|state| state.borrow_mut().admins.push(principal));
    true
}

#[ic_cdk_macros::update (guard = "is_controller") ]
fn remove_admin(principal: Principal) -> Result<bool, String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(index) = state.admins.iter().position(|x| *x == principal) {
            state.admins.remove(index);
            Ok(true)
        } else {
            Err("Admin not found.".to_string())
        }
    } )
}