use std::collections::BTreeMap;

use candid::Principal;

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

    pub fn debit_profit(&mut self, investor: Principal, amount: u64, timestamp: u64, account_id: String) -> Result<(), String> {
        // Check if the investor has enough balance
        let balance = self.get_logs(investor);
        let total_balance_credit: u64 = balance.iter().map(|x| x.amount_e8s).sum();
        let total_balance_debit: u64 = balance.iter().filter_map(|x| {
            match &x.status {
                ProfitStatus::Debit { account_id: _ } => Some(x.amount_e8s),
                _ => None,
            }
        }).sum();
        let total_balance = total_balance_credit - total_balance_debit;
        if total_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        let profit_log = ProfitLogState {
            timestamp: timestamp,
            amount_e8s: amount,
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
        let total_balance_credit: u64 = balance.iter().map(|x| x.amount_e8s).sum();
        let total_balance_debit: u64 = balance.iter().filter_map(|x| {
            match &x.status {
                ProfitStatus::Debit { account_id: _ } => Some(x.amount_e8s),
                _ => None,
            }
        }).sum();
        total_balance_credit - total_balance_debit
    }
}