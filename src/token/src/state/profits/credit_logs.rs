use std::collections::BTreeMap;

use candid::Principal;
use ic_cdk::api::time;

use crate::state::{escrow::EscrowStore, State, TokenState};

use super::store::*;

impl CreditLogsState {
    pub fn new() -> Self {
        Self {
            credit_logs: BTreeMap::new(),
        }
    }

    pub fn add_new_log(
        &mut self,
        log_event: Option<CreditLogEvent>,
        note: Option<String>,
        link: Option<String>,
    ) -> u64 {
        let credit_id = self.credit_logs.last_key_value().map(|f| f.0).unwrap_or(&0) + 1;
        let credit_log = CreditLog {
            credit_id,
            timestamp: time(),
            log_event,
            note,
            link,
            ..Default::default()
        };
        self.add_credit_log(credit_log);
        credit_id
    }

    pub async fn start_sharing_profit(&mut self, credit_id: u64, token_state: TokenState, ledger: Principal) -> Result<(), String> {
        let event = self
            .credit_logs
            .get_mut(&credit_id)
            .map(|f| {
                f.status = CreditLogStatus::Ongoing;
                f.log_event.clone()
            })
            .ok_or("Credit log not found".to_string())?;

        match event {
            Some(event) => match event {
                CreditLogEvent::Equal { amount_e8s } => {
                    let mut memo = 0;
                    for (token_id, token) in token_state.tokens.iter() {
                        let investor = token.owner.principal;
                        self.transfer_profit_to_escrow_of_investor(investor, amount_e8s, &mut memo, ledger).await?;
                        
                    }
                    
                }   
                CreditLogEvent::Selected {
                    token_ids_and_amount_e8s,
                } => {}
                CreditLogEvent::AvailEscrowBalance => {}
            },
            None => {}
        }

        todo!()
    }

    async fn transfer_profit_to_escrow_of_investor(&self, investor: Principal, amount_e8s: u64, memo: &mut u64, ledger: Principal) -> Result<(), String> {
        let from_principal = ic_cdk::id();
        let transfer_to_account_id = State::genral_escrow_account(Some(ic_cdk::id()), investor) ;

             *memo = EscrowStore::transfer_from_escrow(ledger, amount_e8s.saturating_sub(10000), &from_principal, transfer_to_account_id, *memo  ).await.map_err(|f| format!("Failed for investor: {}\n{f}", investor.to_text()))?;
        Ok(())
    }

    fn add_credit_log(&mut self, credit_log: CreditLog) {
        self.credit_logs.insert(credit_log.credit_id, credit_log);
    }

    pub fn get_credit_log(&self, credit_id: u64) -> Option<CreditLog> {
        self.credit_logs.get(&credit_id).cloned()
    }

    pub fn get_credit_logs(&self) -> Vec<&CreditLog> {
        self.credit_logs.values().collect()
    }
}
