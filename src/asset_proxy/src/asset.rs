use candid::Principal;
use ic_cdk::api::call::call;
use ic_cdk_macros::update;
use crate::canisters::get_temp_asset_canister;
use crate::admin::*;
use super::types::*;

#[update(guard = "validate_asset_uploader")]
pub async fn store(asset: AssetStoreArg) -> Result<bool, String> {

    let temp_asset_canister = get_temp_asset_canister().ok_or("Temp asset canister not yet set".to_string())?;

    // Get the temporary asset canister
    // let temp_asset_canister = get_asset_canister(temp_asset_canister);

    // Call the `store` method on the temporary asset canister
    let args = (asset,);
    let result = call(temp_asset_canister, "store", args).await;

    match result {
        Ok(()) => Ok(true),
        Err(e) => Err(format!("Failed to store asset: {:?}", e)),
    }
}

/// Prune function
#[update(guard = "is_controller")]
pub async fn prune(files: Vec<String>) -> Result<bool, String> {
    

    let temp_asset_canister = get_temp_asset_canister().ok_or("Temp asset canister not yet set".to_string())?;

    // Get the temporary asset canister
    // let temp_asset_canister = get_asset_canister(temp_asset_canister);

    // Iterate over the files and call `delete_asset` for each
    for file in files {
        let args = (file.clone(),);
        let result: Result<(), _> = call(temp_asset_canister, "delete_asset", args).await;
        if let Err(e) = result {
            return Err(format!("Failed to delete file '{}': {:?}", file, e));
        }
    }

    Ok(true)
}

/// Reject Files function
#[update(guard = "is_provision_controller")]
pub async fn reject_files(files: Vec<String>) -> Result<bool, String> {
    // Validate the provision canister
    

    let temp_asset_canister = get_temp_asset_canister().ok_or("Temp asset canister not yet set".to_string())?;
    // Get the temporary asset canister
    // let temp_asset_canister = get_asset_canister(temp_asset_canister);

    // Iterate over the files and call `delete_asset` for each
    for file in files {
        let args = (file.clone(),);
        let result: Result<(), _> = call(temp_asset_canister, "delete_asset", args).await;
        if let Err(e) = result {
            return Err(format!("Failed to delete file '{}': {:?}", file, e));
        }
    }

    Ok(true)
}

#[update(guard = "is_provision_controller")]
async fn approve_files(arg: ApproveFilesArg) -> Result<bool, String> {
    
    let asset_canister = arg.asset_canister;
    let temp_asset_canister = get_temp_asset_canister().ok_or("Temp asset canister not yet set".to_string())?;

    ic_cdk::println!("Approve files {:?} {:?}", temp_asset_canister.to_text(), arg);

    // Process files
    for file in &arg.files {
        // Fetch file from temp asset canister
        let (file_data,) :(GetAssetResponse, ) = match check_file_exists(temp_asset_canister,GetAssetArg {
            key: file.clone(),
            accept_encodings: vec![
                "identity".to_string(),
                "gzip".to_string(),
                "br".to_string(),
                "deflate".to_string(),
                "compress".to_string(),
                "zstd".to_string(),
            ],
        } ).await  {
            Ok(f) => f, 
            Err(e) => {
                let _ = check_file_exists(asset_canister,GetAssetArg {
                    key: file.clone(),
                    accept_encodings: vec![
                        "identity".to_string(),
                        "gzip".to_string(),
                        "br".to_string(),
                        "deflate".to_string(),
                        "compress".to_string(),
                        "zstd".to_string(),
                    ],
                } ).await.map_err(|_| e)?;
                return Ok(true);
            }
        };
        
        //  call(
        //     temp_asset_canister,
        //     "get",
        //     (&GetAssetArg {
        //         key: file.clone(),
        //         accept_encodings: vec![
        //             "identity".to_string(),
        //             "gzip".to_string(),
        //             "br".to_string(),
        //             "deflate".to_string(),
        //             "compress".to_string(),
        //             "zstd".to_string(),
        //         ],
        //     },)
        // )
        // .await.map_err(|e| format!("Error fetching file: {:?} with args: {arg:?}", e))?;


        // Store the file in the target asset canister
        let _: () = call(
            asset_canister,
            "store",
            (AssetStoreArg {
                key: file.clone(),
                content_type: file_data.content_type,
                content_encoding: file_data.content_encoding,
                content: file_data.content,
                sha256: file_data
                    .sha256,
            },)
        )
        .await
        .map_err(|e| format!("Error storing file in asset canister: {:?}", e))?;

        // Delete the file from the temp asset canister
        let _:()  = call(
            temp_asset_canister,
            "delete_asset",
            (DeleteAssetArg {
                key: file.clone(),
            },)
        )
        .await
        .map_err(|e| format!("Error deleting file from temp asset canister: {:?}", e))?;
    }

    Ok(true)
}

async fn check_file_exists(canister_id: Principal, args: GetAssetArg ) -> Result<(GetAssetResponse, ), String > {
    Ok(call(
        canister_id,
        "get",
        (&args,)
    )
    .await.map_err(|e| format!("Error fetching file: {:?} with args: {args:?}", e))?)

}

#[ic_cdk_macros::update]
pub async fn get_file(args: GetAssetArg, from_canister: Principal ) -> Result<GetAssetResponse, String> {
    let (file_data,) :(GetAssetResponse, ) = call(
        from_canister,
        "get",
        (&args, )
    )
    .await.map_err(|e| format!("Error fetching file: {:?}", e))?;
    Ok(file_data)

}