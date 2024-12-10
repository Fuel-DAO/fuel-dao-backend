use candid::Principal;
use ic_cdk::api::management_canister::main::{self, CanisterIdRecord}
;


pub async fn delete_canister(canister_id: Principal) -> Result<bool, String> {
    // Step 1: Stop the canister
    if let Err((_, err_msg)) = main::stop_canister(CanisterIdRecord { canister_id }).await {
        return Err(format!("Failed to stop canister: {}", err_msg));
    }

    // Step 2: Delete the canister
    if let Err((_, err_msg)) = main::delete_canister(CanisterIdRecord { canister_id }).await {
        return Err(format!("Failed to delete canister: {}", err_msg));
    }

    Ok(true)
}