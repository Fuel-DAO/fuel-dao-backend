use core::time;
use std::collections::BTreeMap;

use candid::Principal;

use crate::ports::transfer_my_balance_from_escrow;

use super::store::{PorfitStore, ProfitLogState, ProfitStatus};



impl PorfitStore {
    pub fn new() -> Self {
        Self {
            investor_balance: BTreeMap::new(),
        }
    }

    pub fn crefit_profit(&mut self, investor: Principal, profit_log: ProfitLogState) {
        self.investor_balance
            .entry(investor)
            .or_insert_with(Vec::new)
            .push(profit_log);
    }

    pub async fn debit_profit_entry(&mut self, investor: Principal, amount_e8s: u64, account_id: String) -> Result<(), String> {
        // Check if the investor has enough balance
        let timestamp = ic_cdk::api::time();
        
        let total_balance = self.get_current_balance_e8s(investor);
        if total_balance < amount_e8s {
            return Err("Insufficient balance".to_string());
        }
        transfer_my_balance_from_escrow((amount_e8s as f64) / 1e8, account_id.clone()).await?;
        let profit_log = ProfitLogState {
            timestamp: timestamp,
            amount_e8s: amount_e8s,
            status: ProfitStatus::Debit {
                account_id: account_id,
            },
        };
        self.investor_balance
            .entry(investor)
            .or_insert_with(Vec::new)
            .push(profit_log);
        Ok(())
    }

    pub fn get_logs(&self, investor: Principal) -> Vec<ProfitLogState> {
        self.investor_balance
            .get(&investor)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_current_balance_e8s(&self, investor: Principal) -> u64 {
        let balance = self.get_logs(investor);
    
        let mut total_balance_credit = 0u64;
        let mut total_balance_debit = 0u64;
    
        for log in balance {
            match &log.status {
                ProfitStatus::Credit {credit_id:_} => total_balance_credit += log.amount_e8s,
                ProfitStatus::Debit { .. } => total_balance_debit += log.amount_e8s,
            }
        }
    
        total_balance_credit.saturating_sub(total_balance_debit)
    }
    
}