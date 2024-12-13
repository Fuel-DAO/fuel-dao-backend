use candid::{CandidType, Principal, Deserialize};
use ic_cdk::api::call::call;

#[derive(candid::CandidType, candid::Deserialize)]
struct PermissionArgs {
    to_principal: Principal,
    permission: Permission,
}

#[derive(candid::CandidType, candid::Deserialize)]
enum Permission {
    ManagePermissions,
    Commit,
    Revoke
}

pub async fn grant_asset_admin_perms(
    canister: Principal,
    user: Principal,
) -> Result<bool, String> {
    // Construct the permission arguments
    let args = &PermissionArgs {
        to_principal: user,
        permission: Permission::ManagePermissions,
    };

    // Call the `grant_permission` method on the asset canister
    match call(canister, "grant_permission", (args, )).await {
        Ok(()) => Ok(true),
        Err((_, err_msg)) => Err(format!("Failed to grant permission: {}", err_msg)),
    }
}
pub async fn grant_asset_edit_perms(
    canister: Principal,
    user: Principal,
) -> Result<bool, String> {
    // Construct the permission arguments
    let args = &PermissionArgs {
        to_principal: user,
        permission: Permission::Commit,
    };

    // Call the `grant_permission` method on the asset canister
    match call(canister, "grant_permission", (args, )).await {
        Ok(()) => Ok(true),
        Err((_, err_msg)) => Err(format!("Failed to grant permission: {}", err_msg)),
    }
}

pub async fn revoke_asset_edit_perms(
    canister: Principal,
    user: Principal,
) -> Result<bool, String> {
    // Construct the permission arguments
    let args = &PermissionArgs {
        to_principal: user,
        permission: Permission::Revoke,
    };

    // Call the `grant_permission` method on the asset canister
    match call(canister, "grant_permission", (args, )).await {
        Ok(()) => Ok(true),
        Err((_, err_msg)) => Err(format!("Failed to revoke permission: {}", err_msg)),
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ApproveFilesArg {
    pub files: Vec<String>,
    pub asset_canister: Principal,
}

pub async fn approve_files_from_proxy(canister: Principal, files: Vec<String>, asset_proxy_canister: Principal) -> Result<bool, String> {

    // Prepare the argument
    let args = ApproveFilesArg {
        files,
        asset_canister: canister,
    };

    // Call the approve_files method on the asset proxy canister
    let (result, ) = call(asset_proxy_canister, "approve_files", (args,))
        .await
        .map_err(|e| format!("Error calling approve_files: {:?}", e))?;
    ic_cdk::println!("Approve files {result:?}");
    // Return success if no error occurred
    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}