
use candid::{CandidType, Deserialize, };

use super::metadata::Metadata;
use super::escrow::EscrowStore;
use super::profits::store::CreditLogsState;
use super::transactions::TxnIndexStore;
use super::TokenState;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct State {
    pub metadata: Option<MetaDataState>, 
    pub escrow: EscrowStore,
    pub transactions: TxnIndexStore,
    pub tokens: TokenState, 
    pub profit_transfer_and_logs: Option<CreditLogsState>
}

#[derive(CandidType, Deserialize, Clone)]
pub struct MetaDataState {
    pub metadata: Metadata, 
    pub total_supply: u64
}

impl MetaDataState {
    pub fn increment_supply(&mut self) {
        self.total_supply += 1;
    }
    pub fn decrement_supply(&mut self) {
        self.total_supply -= 1;
    }
}

