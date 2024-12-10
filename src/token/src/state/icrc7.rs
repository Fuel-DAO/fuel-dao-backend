use candid::{CandidType, Nat};

use crate::STATE;

#[ic_cdk_macros::query]
pub fn icrc7_max_query_batch_size() -> Option<Nat> {
    None
}

#[ic_cdk_macros::query]
pub fn icrc7_max_update_batch_size() -> Option<Nat> {
    None
}

#[ic_cdk_macros::query]
pub fn icrc7_max_default_take_value() -> Option<Nat> {
    None
}

#[ic_cdk_macros::query]
pub fn icrc7_max_take_value() -> Option<Nat> {
    None
}

#[ic_cdk_macros::query]
pub fn icrc7_max_memo_size() -> Option<Nat> {
    None
}

#[ic_cdk_macros::query]
pub fn icrc7_atomic_batch_transfers() -> Option<bool> {
    None
}

#[ic_cdk_macros::query]
pub fn icrc7_tx_window() -> Option<Nat> {
    None
}

#[ic_cdk_macros::query]
pub fn icrc7_permitted_drift() -> Option<Nat> {
    None
}

// Functions
#[ic_cdk_macros::query]
pub fn icrc7_symbol() -> String {
    STATE.with(|store| {
        let metadata = store.borrow().metadata.clone();
        if metadata.is_none() {
            return String::new();
        }

        metadata.unwrap().metadata.symbol
    })
}

#[ic_cdk_macros::query]
pub fn icrc7_name() -> String {
    STATE.with(|store| {
        let metadata = store.borrow().metadata.clone();
        if metadata.is_none() {
            return String::new();
        }

        metadata.unwrap().metadata.name
    })
}

#[ic_cdk_macros::query]
pub fn icrc7_description() -> Option<String> {
    STATE.with(|store| {
        let metadata = store.borrow().metadata.clone();
        if metadata.is_none() {
            return None;
        }

        Some(metadata.unwrap().metadata.description)
    })
}

#[ic_cdk_macros::query]
pub fn icrc7_logo() -> Option<String> {
    STATE.with(|store| {
        let metadata = store.borrow().metadata.clone();
        if metadata.is_none() {
            return None;
        }

        Some(metadata.unwrap().metadata.logo)
    })
}

#[ic_cdk_macros::query]
pub fn icrc7_total_supply() -> Nat {
    STATE.with(|store| {
        let metadata = store.borrow().metadata.clone();
        if metadata.is_none() {
            return Nat::from(0u8);
        }
        metadata.unwrap().total_supply.into()
    })
}

#[ic_cdk_macros::query]
pub fn icrc7_supply_cap() -> Option<Nat> {
    STATE.with(|store| {
        let metadata = store.borrow().metadata.clone();
        if metadata.is_none() {
            return None;
        }

        Some(metadata.unwrap().metadata.supply_cap.into())
    })
}

// Metadata Query Result
pub type ICRC7MetadataQueryResult = Vec<(String, MetadataValue)>;

#[derive(CandidType)]
pub enum MetadataValue {
    Text(String),
    Nat(Nat),
}

#[ic_cdk_macros::query]
pub fn icrc7_collection_metadata() -> ICRC7MetadataQueryResult {
    STATE.with(|store| {
        let metadata = store.borrow().metadata.clone();
        if metadata.is_none() {
            return Vec::new();
        }

        let state = metadata.unwrap();
        let metadata = state.metadata;
        let total_supply = state.total_supply;

        vec![
            (
                "icrc7:name".to_string(),
                MetadataValue::Text(metadata.name.clone()),
            ),
            (
                "icrc7:symbol".to_string(),
                MetadataValue::Text(metadata.symbol.clone()),
            ),
            (
                "icrc7:total_supply".to_string(),
                MetadataValue::Nat(total_supply.clone().into()),
            ),
            (
                "icrc7:supply_cap".to_string(),
                MetadataValue::Nat(metadata.supply_cap.clone().into()),
            ),
            (
                "icrc7:description".to_string(),
                MetadataValue::Text(metadata.description.clone()),
            ),
            (
                "icrc7:logo".to_string(),
                MetadataValue::Text(metadata.logo.clone()),
            ),
        ]
    })
}
