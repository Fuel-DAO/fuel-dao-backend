use candid::Principal;

use crate::STATE;


pub fn is_controller() -> Result<(), String> {
    // let caller = ic_cdk::caller();
    // if ic_cdk::api::is_controller(&caller){ return  Ok(());}
    // STATE.with(|state| {
    //     if state
    //         .borrow()
    //         .controllers
    //         .contains(&ic_cdk::caller())
    //     {
    //         Ok(())
    //     } else {
    //         Err("You are not authorized to perform this action.".to_string())
    //     }
    // })
    Ok(())
}


#[ic_cdk_macros::query (guard = "is_controller") ]
fn get_controllers() -> Vec<Principal> {
    STATE.with(|state| state.borrow().controllers.clone())
}


#[ic_cdk_macros::update(guard = "is_controller")]
pub fn add_controller(new_controller: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if !state.controllers.contains(&new_controller) {
            state.controllers.push(new_controller);
            Ok(())
        } else {
            Err("Controller already exists.".to_string())
        }
    })
}

#[ic_cdk_macros::update(guard = "is_controller")]
pub fn remove_controller(controller_to_remove: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(index) = state.controllers.iter().position(|x| *x == controller_to_remove) {
            state.controllers.remove(index);
            Ok(())
        } else {
            Err("Controller not found.".to_string())
        }
    })
}
