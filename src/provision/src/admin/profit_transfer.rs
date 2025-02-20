use std::collections::BTreeMap;
use crate::admin::admin::is_controller;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::call;
use serde::Serialize;


#[derive(CandidType, Serialize, Deserialize,   Debug, Clone)]
pub enum CreditLogEvent {
    Equal {
        amount_e8s: u64,
    },
    Selected {
        token_ids_and_amount_e8s: BTreeMap<TokenId, u64>,
    },
    AvailEscrowBalance,
}

pub type TokenId = u32;

#[ic_cdk_macros::update(guard = "is_controller")]
pub async fn create_admin_profit_transfer_log(canister_id: Principal, log_event: Option<CreditLogEvent>,
    note: Option<String>,
    link: Option<String>,) -> Result<u64, String> {

    let args = (log_event, note, link,);
    let (result,) = call(canister_id, "create_admin_profit_transfer_log", args).await.map_err(|f| format!("Failed to create admin profit transfer log: {:?}", f))? ;
        result
}

#[ic_cdk_macros::update(guard = "is_controller")]
pub async fn create_investor_profit_transfer_and_log(canister_id: Principal, credit_id: u64) -> Result<CreditLogEvent, String> {

    let args = (credit_id,);
    let (result,) = call(canister_id, "create_investor_profit_transfer_and_log", args).await.map_err(|f| format!("Failed to transfer profit or create transfer log: {:?}", f))? ;
        result
}