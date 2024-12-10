use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use super::CollectionRequest;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct  CollectionRequestConfig {
    pub request: CollectionRequest,
    pub config: CollectionConfig,
}


#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct CollectionConfig {
    pub collection_owner: Principal,
    pub approval_status: ConfigStatus,
    pub token_canister: Option<Principal>,
    pub asset_canister: Option<Principal>,
}

impl CollectionConfig {
    pub fn new_pending() -> Self {
        Self { collection_owner:  ic_cdk::caller(), approval_status: ConfigStatus::Pending, token_canister: None, asset_canister:None }
    }

    pub fn is_pending(&self) -> bool {
        self.approval_status == ConfigStatus::Pending
    }

    pub fn reject_request(&mut self) {
        
    }
}


#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum  ConfigStatus {
    Pending, 
    Approved, 
    Rejected
}