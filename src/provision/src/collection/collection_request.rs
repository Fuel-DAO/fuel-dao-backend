use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::{
    admin::admin::is_controller,
    asset_permission::{
        approve_files_from_proxy, grant_asset_admin_perms, grant_asset_edit_perms,
        revoke_asset_edit_perms,
    },
    token::deploy_token,
    STATE,
};

use super::{CollectionConfig, CollectionRequestConfig, ConfigStatus};
use crate::canisters::delete_canister::delete_canister;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct CollectionRequest {
    pub weight: f64,
    pub drive_type: String,
    pub purchase_price: u128,
    pub token: Principal,
    pub documents: Vec<(String, String)>,
    pub supply_cap: u128,
    pub displays: String,
    pub seating: String,
    pub cargo: f64,
    pub logo: String,
    pub name: String,
    pub overall_height: f64,
    pub description: String,
    pub overall_width: f64,
    pub track_front: f64,
    pub ground_clearance: f64,
    pub key_features: Vec<String>,
    pub range_per_charge: f64,
    pub track_rear: f64,
    pub acceleration: String,
    pub charging_speed: String,
    pub wheels: f64,
    pub brochure_url: String,
    pub index: Principal,
    pub price: f64,
    pub battery: String,
    pub overall_length: f64,
    pub symbol: String,
    pub treasury: Principal,
    pub images: Vec<String>,
}

#[ic_cdk_macros::update]
pub fn add_collection_request(collection: CollectionRequest) -> Result<u64, String> {
    STATE.with(|f| {
        let mut state = f.borrow_mut();
        let id = state
            .collection_requests
            .last_key_value()
            .map(|f| f.0)
            .unwrap_or(&0)
            + 1;
        state.collection_requests.insert(
            id,
            CollectionRequestConfig {
                request: collection,
                config: CollectionConfig::new_pending(),
            },
        );
        Ok(id)
    })
}

#[ic_cdk_macros::query]
pub fn get_request_info(id: u64) -> Option<CollectionRequest> {
    STATE.with(|f| {
        f.borrow()
            .collection_requests
            .get(&id)
            .cloned()
            .map(|f| f.request)
    })
}

#[ic_cdk_macros::query]
pub fn get_pending_requests() -> Vec<u64> {
    STATE.with(|f| {
        let state = f.borrow();
        state
            .collection_requests
            .iter()
            .filter_map(|(&id, request_config)| {
                if request_config.config.is_pending() {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    })
}

/*
export function reject_request(id: nat): Result<bool, text> {
  const validationResult = validateAdmin(ic.caller());
  if (validationResult.Err) return validationResult;

  const requestConfig = RequestStore.config.get(id);
  if (!requestConfig) return Result.Err("No request exists with the given id.");
  if (requestConfig.approval_status.Pending === undefined)
    return Result.Err("Request already processed.");

  RequestStore.rejectRequest(id);
  return Result.Ok(true);
}
*/

// Step 1: Validate Controller
#[ic_cdk_macros::update(guard = "is_controller")]
pub async fn delete_collection(request_id: u64) -> Result<bool, String> {
    // Step 2: Access the collection config
    let collection_config = STATE.with(|state| {
        let state = state.borrow();
        state.collection_requests.get(&request_id).cloned()
    });

    if collection_config.is_none() {
        return Err("No collection exists with the given ID.".to_string());
    }

    let collection_config = collection_config.unwrap();

    // Step 3: Check approval status
    match collection_config.config.approval_status {
        ConfigStatus::Pending => {
            return Err("Collection hasn't received admin approval.".to_string());
        }
        ConfigStatus::Approved => {
            // Step 4: Delete the token canister if it exists
            if let Some(token_canister) = collection_config.config.token_canister {
                if let Err(err_msg) = delete_canister(token_canister).await {
                    return Err(err_msg);
                }
            }

            // Step 5: Delete the asset canister if it exists
            if let Some(asset_canister) = collection_config.config.asset_canister {
                if let Err(err_msg) = delete_canister(asset_canister).await {
                    return Err(err_msg);
                }
            }
        }
        _ => {}
    }

    // Step 6: Remove the collection request
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.collection_requests.remove(&request_id);
    });

    Ok(true)
}

#[ic_cdk_macros::update(guard = "is_controller")]
pub fn reject_request(id: u64) -> Result<bool, String> {
    STATE.with(|f| {
        let mut state = f.borrow_mut();
        state
            .collection_requests
            .remove(&id)
            .map(|_| Ok(true))
            .unwrap_or(Err("No request exists with the given id.".into()))
    })
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct ListCollection {
    pub id: u64,
    pub token_canister: Principal,
    pub asset_canister: Principal,
}

#[ic_cdk_macros::query]
pub fn list_collections() -> Vec<ListCollection> {
    // Step 1: Access the state to retrieve collection requests
    STATE.with(|state| {
        let state = state.borrow();

        // Step 2: Filter and map collection requests with a valid token canister
        state
            .collection_requests
            .iter()
            .filter_map(|(&id, config)| {
                // Only include entries where `token_canister` is `Some`
                if let Some(token_canister) = config.config.token_canister {
                    Some(ListCollection {
                        id,
                        token_canister,
                        asset_canister: config
                            .config
                            .asset_canister
                            .unwrap_or(Principal::anonymous()),
                    })
                } else {
                    None
                }
            })
            .collect()
    })
}


#[ic_cdk_macros::update(guard = "is_controller")]
pub async fn approve_request(id: u64) -> Result<ListCollection, String> {
    let mut state = STATE.with(|f| f.borrow_mut().clone());
    let collection = match state.collection_requests.get_mut(&id) {
        Some(c) => c,
        None => return Err("Invalid collection Request".into()),
    };

    if collection.config.approval_status == ConfigStatus::Approved {
      return Ok(ListCollection { id: id, token_canister: collection.config.token_canister.unwrap() , asset_canister: collection.config.asset_canister.unwrap() })
    }

    // let wasm = include_bytes!("../../../../wasm/asset/assetstorage.wasm.gz");

     let wasm= match state.asset_wasm {
        Some(wasm) =>wasm,
        None => return Err("Asset wasm not set".into()),
    } ;

    let deploy_asset_result = match collection.config.asset_canister {
        Some(p) => p,
        None => crate::canisters::assets::deploy_asset(wasm.to_vec()).await?,
    };

    collection.config.asset_canister = Some(deploy_asset_result);

    STATE.with_borrow_mut(|f| {
       match  f.collection_requests.get_mut(&id) {
        Some(collection) => {
            collection.config.asset_canister = Some(deploy_asset_result);
        },
        None => {}
       }
    });

    // Step 3: Deploy the asset canister
    let asset_canister_id = deploy_asset_result;

    let asset_proxy_canister = state
        .asset_proxy_canister
        .ok_or(String::from("Asset Proxy canister not set"))?;

    // // Step 4: Grant proxy permissions ///TODO:// Use
    grant_asset_edit_perms(asset_canister_id, asset_proxy_canister).await?;

    let request = collection.request.clone();
    // // Step 5: Prepare the files for approval
    let approved_files: Vec<String> = collection
        .request
        .documents
        .iter()
        .map(|doc| doc.1.clone())
        .chain(collection.request.images.clone())
        .chain(if !&request.logo.is_empty() {
            vec![request.logo.clone()]
        } else {
            vec![]
        })
        .collect();

    // // Step 6: TODO:// Approve the files
    // approve_files_from_proxy(asset_canister_id, approved_files, asset_proxy_canister).await?;

    // // Step 7: Revoke proxy permissions
    revoke_asset_edit_perms(
        asset_canister_id,
        state.asset_proxy_canister.unwrap_or(Principal::anonymous()),
    )
    .await?;

    // // Step 8: Deploy the token canister
    let collection_owner = collection.config.collection_owner.clone();
    let asset_canister = asset_canister_id;
    let mut token_metadata = request
        .clone()
        .into_metadata(collection_owner, asset_canister);

    if !request.logo.is_empty() {
        token_metadata.logo = format!(
            "https://{}.icp0.io{}",
            asset_canister_id.to_string(),
            request.logo
        );
    }

    // let wasm = include_bytes!("../../../../wasm/token/token.wasm.gz").to_vec();
    let wasm= match state.token_wasm     {
        Some(wasm) =>wasm,
        None => return Err("Token wasm not set".into()),
    } ;

    let deploy_token_result = match collection.config.token_canister {
        Some(p) => p,
        None => deploy_token(wasm, token_metadata).await?,
    };

    let token_canister_id = deploy_token_result;

    collection.config.token_canister = Some(token_canister_id);

    STATE.with_borrow_mut(|f| {
        match  f.collection_requests.get_mut(&id) {
         Some(collection) => {
             collection.config.token_canister = Some(token_canister_id);
         },
         None => {}
        }
     });

    collection.config.approval_status = ConfigStatus::Approved;

    // // Step 9: Grant admin and edit permissions
    let _ = grant_asset_admin_perms(asset_canister_id, token_canister_id).await?;
    //     .map_err(|err| ApproveError::GrantPermissionsError(err))?;

    grant_asset_edit_perms(asset_canister_id, collection_owner.clone()).await?;
    //     .map_err(|err| ApproveError::GrantPermissionsError(err))?;

    STATE.with_borrow_mut(|f| {
        let request = f.collection_requests.get_mut(&id);
        if let Some(req) = request {
            req.config.approval_status = ConfigStatus::Approved;
            req.config.asset_canister = Some(asset_canister_id);
            req.config.token_canister = Some(token_canister_id);
        }
    });

    // // Step 11: Return the success response
    Ok(ListCollection {
        id,
        asset_canister: asset_canister_id,
        token_canister: token_canister_id,
    })
}
