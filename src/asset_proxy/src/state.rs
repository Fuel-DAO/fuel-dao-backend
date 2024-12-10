use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;



#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct State {
   pub temp_asset_canister:  Option<Principal>,
   pub provision_canister:  Option<Principal>,
   pub admins: Vec<Principal>,
}