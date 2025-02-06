use std::collections::BTreeMap;

use candid::{CandidType, Principal,Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreditLogsState {
    pub credit_logs: BTreeMap<u64, CreditLog>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Default, Clone)]
pub struct  CreditLog {
    pub credit_id: u64,
    pub timestamp: u64,
    pub log_event: Option<CreditLogEvent>,
    pub status: CreditLogStatus,
    pub note: Option<String>,
    pub link: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Default, Clone)]
pub enum CreditLogStatus {
    #[default]
    Pending,
    Ongoing,
    Completed,
}

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


#[derive(CandidType, Serialize, Deserialize, Debug, Default, Clone)]
pub struct PorfitStore {
    pub investor_balance: BTreeMap<Principal, Vec<ProfitLogState>>,
}

#[derive(CandidType, Serialize, Deserialize, Debug,  Clone)]
pub enum ProfitStatus {
    Credit {
        credit_id: u64,
    }, 
    Debit {
        account_id: String,
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct ProfitLogState {
   pub timestamp: u64,
   pub amount_e8s: u64,
   pub status: ProfitStatus,
}