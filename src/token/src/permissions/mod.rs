use candid::Principal;
use ic_cdk::api::call::call;

#[derive(candid::CandidType, candid::Deserialize)]
struct PermissionArgs {
    to_principal: Principal,
    permission: Permission,
}

#[derive(candid::CandidType, candid::Deserialize)]
enum Permission {
    Commit,
    Revoke
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
        permission: Permission::Commit,
    };

    // Call the `grant_permission` method on the asset canister
    match call(canister, "grant_permission", (args, )).await {
        Ok(()) => Ok(true),
        Err((_, err_msg)) => Err(format!("Failed to revoke permission: {}", err_msg)),
    }
}
