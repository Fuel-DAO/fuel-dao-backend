use ic_cdk::api::management_canister::{
            main::{
                create_canister, install_code, CreateCanisterArgument,  InstallCodeArgument
            },
            provisional::CanisterSettings,
        }
;
use candid::{CandidType, Deserialize,  Principal};
use serde::Serialize;

#[derive(CandidType, Deserialize)]
pub struct AssetCanisterArgs {
    pub init: (),
}

pub async fn deploy_asset(wasm: Vec<u8>) -> Result<Principal, String> {
    // Step 1: Create a new canister with updated CanisterSettings
    let canister_id = match create_canister(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers:Some(vec![ic_cdk::api::id()]), ..Default::default()
            }),
        },
        /* 14_000_000_000, */ 500_000_000_000 * 2,
    )
    .await
    {
        Ok(response) => response.0.canister_id,
        Err((_, err_msg)) => return Err(format!("Failed to create canister: {}", err_msg)),
    };

    // Step 2: Install chunked code on the created canister
    let install_code_arg = InstallCodeArgument {
        mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Install,
        canister_id,
        wasm_module:wasm,
        arg: candid::encode_args((AssetCanisterArgs { init: () },)).unwrap(),
    };

    if let Err((_, err_msg)) =  install_code(install_code_arg).await {
        return Err(format!("Failed to install code: {}", err_msg));
    }

    // Step 3: Return the created canister ID
    Ok(canister_id)
}


#[derive(CandidType, Deserialize)]
struct GrantPermissionArgs {
    to_principal: Principal,
    permission: Permission,
}

#[derive(CandidType, Deserialize, Serialize)]
enum Permission {
    Commit,
}