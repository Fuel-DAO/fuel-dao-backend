use candid::Principal;
use ic_cdk::api::time;

use crate::state::{escrow::EscrowStore, State, TokenState};
use super::store::*;

impl CreditLogsState {
    pub fn add_new_log(
        &mut self,
        log_event: Option<CreditLogEvent>,
        note: Option<String>,
        link: Option<String>,
    ) -> u64 {
        // Generate a new credit_id by taking the last key value and adding one.
        let credit_id = self.credit_logs.last_key_value().map(|(k, _)| *k).unwrap_or(0) + 1;
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

    /// Starts the profit-sharing process.
    /// If the credit log is already completed, it returns the stored event.
    /// Otherwise, it marks the log as ongoing and then completes any missing transfers.
    pub async fn start_sharing_profit(
        &mut self,
        credit_id: u64,
        token_state: TokenState,
        ledger: Principal,
    ) -> Result<CreditLogEvent, String> {
        // Early return if this credit log has already been completed.
        if let Some(log) = self.credit_logs.get(&credit_id) {
            if log.status == CreditLogStatus::Completed {
                return log
                    .log_event
                    .clone()
                    .ok_or_else(|| "Credit log event missing".to_string());
            }
        }
    
        // Mark the log as Ongoing.
        {
            let credit_log = self
                .credit_logs
                .get_mut(&credit_id)
                .ok_or("Credit log not found".to_string())?;
            credit_log.status = CreditLogStatus::Ongoing;
        }
    
        // Attempt to complete any ongoing credit by checking for missing investor credits.
        self.complete_ongoing_credit(credit_id, &token_state, ledger).await?;
    
        // Return the credit log event.
        self.credit_logs
            .get(&credit_id)
            .and_then(|log| log.log_event.clone())
            .ok_or_else(|| "Credit log event missing".to_string())
    }
    
    /// Determines which investors are missing their credit entry for the given credit log.
    pub fn get_failed_transfers(
        &self,
        credit_id: u64,
        token_state: &TokenState,
    ) -> Vec<(Principal, u64)> {
        let mut failed = Vec::new();
        if let Some(credit_log) = self.credit_logs.get(&credit_id) {
            match &credit_log.log_event {
                Some(CreditLogEvent::Equal { amount_e8s }) => {
                    // Every investor in the token state should have a credit entry.
                    for (_, token) in token_state.tokens.iter() {
                        let investor = token.owner.principal;
                        let has_credit = self
                            .investors_credit_logs
                            .get_logs(investor)
                            .iter()
                            .any(|log| match log.status {
                                ProfitStatus::Credit { credit_id: cid } if cid == credit_id => true,
                                _ => false,
                            });
                        if !has_credit {
                            failed.push((investor, *amount_e8s));
                        }
                    }
                }
                Some(CreditLogEvent::Selected { token_ids_and_amount_e8s }) => {
                    // Only selected investors should have a credit entry.
                    for (token_id, amount) in token_ids_and_amount_e8s.iter() {
                        if let Some(token) = token_state.tokens.get(token_id) {
                            let investor = token.owner.principal;
                            let has_credit = self
                                .investors_credit_logs
                                .get_logs(investor)
                                .iter()
                                .any(|log| match log.status {
                                    ProfitStatus::Credit { credit_id: cid } if cid == credit_id => true,
                                    _ => false,
                                });
                            if !has_credit {
                                failed.push((investor, *amount));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        failed
    }
    
    /// Completes any missing transfers for a credit log by retrying for those investors.
    /// Once all expected credits are present, the credit log is marked as Completed.
    pub async fn complete_ongoing_credit(
        &mut self,
        credit_id: u64,
        token_state: &TokenState,
        ledger: Principal,
    ) -> Result<(), String> {
        let failed = self.get_failed_transfers(credit_id, token_state);
        if failed.is_empty() {
            if let Some(log) = self.credit_logs.get_mut(&credit_id) {
                log.status = CreditLogStatus::Completed;
            }
            return Ok(());
        }
    
        let mut memo = 0;
        for (investor, amount) in failed.iter() {
            self.transfer_profit_to_escrow_of_investor(*investor, *amount, &mut memo, ledger)
                .await
                .map_err(|err| format!("Failed for investor {}: {}", investor.to_text(), err))?;
            self.investors_credit_logs.crefit_profit(
                *investor,
                ProfitLogState {
                    timestamp: time(),
                    amount_e8s: *amount,
                    status: ProfitStatus::Credit { credit_id },
                },
            );
        }
    
        // Verify all transfers succeeded.
        if self.get_failed_transfers(credit_id, token_state).is_empty() {
            if let Some(log) = self.credit_logs.get_mut(&credit_id) {
                log.status = CreditLogStatus::Completed;
            }
            Ok(())
        } else {
            Err("Some transfers still failed after retry".to_string())
        }
    }
    
    /// Handles the actual profit transfer for a given investor.
    async fn transfer_profit_to_escrow_of_investor(
        &self,
        investor: Principal,
        amount_e8s: u64,
        memo: &mut u64,
        ledger: Principal,
    ) -> Result<(), String> {
        let from_principal = ic_cdk::id();
        let transfer_to_account_id = State::genral_escrow_account(Some(ic_cdk::id()), investor);
        *memo = EscrowStore::transfer_from_escrow(
            ledger,
            amount_e8s.saturating_sub(10_000),
            &from_principal,
            transfer_to_account_id,
            *memo,
        )
        .await
        .map_err(|f| format!("Failed for investor: {}\n{}", investor.to_text(), f))?;
        Ok(())
    }

    fn add_credit_log(&mut self, credit_log: CreditLog) {
        self.credit_logs.insert(credit_log.credit_id, credit_log);
    }

    pub fn get_credit_log(&self, credit_id: u64) -> Option<CreditLog> {
        self.credit_logs.get(&credit_id).cloned()
    }

    pub fn get_credit_logs(&self) -> Vec<CreditLog> {
        self.credit_logs.values().cloned().collect()
    }
}
