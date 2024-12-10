#![allow(dead_code, unused_imports)]

use crate::{state::{icrc1, Owner}, validations, STATE};

use super::{
    escrow::{EscrowStore, SaleStatus}, metadata::Metadata, models::*, subaccount::{AccountIdentifier, Subaccount}, State, TokenState
};
use candid::{self, CandidType, Decode, Deserialize, Encode, Nat, Principal};
use ic_cdk::{api::call::CallResult, caller};
use ic_ledger_types::{Memo,  Tokens, DEFAULT_SUBACCOUNT};
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
impl State {
    pub async fn accept_sale(&self) -> Result<bool, String> {
        // Check if the sale is live
        let escrow_store = self.escrow.clone();
        if escrow_store.sale_status != SaleStatus::Live {
            return Err("Sale not live.".to_string());
        }


        // Retrieve treasury, ledger, and booked tokens
        let metadata = self.metadata.clone().map(|f| f.metadata);
        if metadata.is_none() {
            return Err("Metadata not set".into());
        }
        let metadata = metadata.unwrap();
        let treasury = metadata.treasury;
        let ledger = metadata.token;
        let booked_tokens = escrow_store.get_booked_tokens();

        for (investor, quantity) in booked_tokens.iter().filter(|f| f.1 > &0) {
            let price = metadata.price;
            let user_invested_amount = quantity.clone() as u64 * (price) as u64;
            // Transfer funds to treasury
            const TRANSFER_FEE: u64 = 10_000;

            let args  =TransferArg {
                from_subaccount: Some(Subaccount::from(&investor.clone()).0),
                to: Account {
                    owner: treasury,
                    subaccount: None,
                },
                fee: Some(TRANSFER_FEE.into()),
                created_at_time: None,
                memo: None,
                amount: user_invested_amount.into(),
            };

            let transaction = icrc1::icrc1_transfer(ledger, args.clone()).await.map_err(|f| format!("Failed to do icrc1 transfer: {quantity} * {price} = {user_invested_amount} {f:?} {args:?}"))?;

            // let args = TransferArgs {
            //     to: Icrc1Account {
            //         owner: treasury,
            //         subaccount: None,
            //     },
            //     from_subaccount: Some(Subaccount::from(&investor.clone()).to_vec()),
            //     fee: Some(TRANSFER_FEE.clone().into()),
            //     memo: None,
            //     created_at_time: None,
            //     amount: user_invested_amount as u64,
            // };

            // // let encoded_args = &args/*  Encode!(&args).expect("Failed to encode arguments") */;
            
            // // let args = ic_ledger_types::TransferArgs {
            // //   memo: Memo(0),
            // //   amount: Tokens::from_e8s(user_invested_amount as u64),
            // //   fee: Tokens::from_e8s(TRANSFER_FEE),
            // //   from_subaccount: Some(ic_ledger_types::Subaccount::from(investor.clone())),
            // //   to: ic_ledger_types::AccountIdentifier::from_hex(&AccountIdentifier::from_principal(treasury, None).to_hex())?,
            // //   created_at_time: None,
            // // };
            // let encoded_args = args.clone()/*  Encode!(&args).expect("Failed to encode arguments") */;

            // let (_transfer_result,): (Result<Nat, ic_ledger_types::TransferError> , ) = ic_cdk::call(
            //     ledger,
            //     "icrc1_transfer",
            //     (encoded_args,),
            // )
            // .await
            // .map_err(|err| format!("Icrc1 Transfer call failed: {err:?} for owner: {}, invester: {}, args: {args:?}", treasury.to_text(), investor.to_text(), ))?;

            // _transfer_result.map_err(|e| format!("Icrc1 Transfer failed {e:?}") )?;

            
            

            // Mint tokens for the investor
            for _ in 0..quantity.clone() {
                STATE.with_borrow_mut(|f|{
                    f.tokens.mint(
                        *investor,
                        Some(Subaccount::from(investor).to_vec()),
                    );
                    f.metadata.as_mut().map(|f| f.increment_supply());
                } );
                
            }
        }

         // Accept the sale
        STATE.with_borrow_mut(|f| f.escrow.accept_sale());

        Ok(true)
    }
    pub async fn accept_sale_individual_icrc1_transfer(invester: Principal, quantity: u128, metadata: Option<Metadata>, sale_status: SaleStatus ) -> Result<bool, String> {
        // Check if the sale is live
        if sale_status != SaleStatus::Live {
            return Err("Sale not live.".to_string());
        }
        // Retrieve treasury, ledger, and booked tokens
        if metadata.is_none() {
            return Err("Metadata not set".into());
        }
        let metadata = metadata.unwrap();

            let price = metadata.price;
            let ledger =  metadata.token;
            let treasury  = metadata.treasury;
            let user_invested_amount = quantity.clone() as u64 * (price) as u64;
            // Transfer funds to treasury
            const TRANSFER_FEE: u64 = 10_000;

            let args  =TransferArg {
                from_subaccount: Some(Subaccount::from(&invester.clone()).0),
                to: Account {
                    owner: treasury,
                    subaccount: None,
                },
                fee: Some(TRANSFER_FEE.into()),
                created_at_time: None,
                memo: None,
                amount: user_invested_amount.into(),
            };

            let _ = icrc1::icrc1_transfer(ledger, args.clone()).await.map_err(|f| format!("Failed to do icrc1 transfer: {quantity} * {price} = {user_invested_amount} {f:?} {args:?}"))?;

        Ok(true)
    }
    

    pub async fn get_excess_escrow_balance(&self) -> Result<Vec<Principal>, String> {

        let metadata = self.get_metadata().await?; // Assume this retrieves the Metadata struct

        let escrow_store = self.escrow.clone(); // Assume this retrieves the EscrowStore instance

        if escrow_store.sale_status == SaleStatus::Live {
            return Err("Sale is live.".to_string());
        }

        let mut excess = Vec::new();

        for principal in escrow_store.get_participating_investors() {

            let subaccount = Subaccount::from(&principal);
        let icp_ledger = metadata.token;

        let escrow_balance = EscrowStore::icrc1_balance_of(
            icp_ledger,
            Icrc1Account {
                owner: ic_cdk::id(),
                subaccount: Some(subaccount.to_vec()),
            },
        )
        .await?;

        let total_invested_count = escrow_store
            .get_booked_tokens()
            .get(&principal)
            .cloned()
            .unwrap_or_else(|| 0);

        let total_cost = (total_invested_count ) as f64
            * &(metadata.price);

        if (escrow_balance as f64) > total_cost {
            excess.push(principal);
        }

        }

        Ok(excess)


    }

    /// Should not be anonymous
    pub async fn book_tokens(&self, arg: BookTokensArg) -> Result<bool, String> {
        let principal = caller();
        let metadata = self.get_metadata().await?; // Assume this retrieves the Metadata struct

        let mut escrow_store = self.escrow.clone(); // Assume this retrieves the EscrowStore instance

        if escrow_store.sale_status != SaleStatus::Live {
            return Err("Sale not live.".to_string());
        }

        if arg.quantity <= 0 {
            return Err("Quantity should be at least 1.".to_string());
        }

        

        let subaccount = Subaccount::from(&principal);
        let icp_ledger = metadata.token;

        let escrow_balance = EscrowStore::icrc1_balance_of(
            icp_ledger,
            Icrc1Account {
                owner: ic_cdk::id(),
                subaccount: Some(subaccount.to_vec()),
            },
        )
        .await?;

        let total_invested_count = escrow_store
            .get_booked_tokens()
            .get(&principal)
            .cloned()
            .unwrap_or_else(|| 0);

        let total_cost = ((&total_invested_count + &(arg.quantity as u128)) as f64)
            * &(metadata.price + 10_000.0);

        if (escrow_balance as f64) < total_cost {
            return Err(format!("Invalid balance in escrow. Req quantity: {} Total invested: {total_invested_count} Current balanace: {escrow_balance}, total cost in e8s: {total_cost}", arg.quantity));
        }

        ic_cdk::println!("Escrow balance {escrow_balance}, cost {total_cost} ");

        if &escrow_store.total_booked_tokens + &(arg.quantity as u128) > metadata.supply_cap {
            return Err("Supply cap reached.".to_string());
        }

         escrow_store.book_tokens(principal, arg.quantity.into());

        Ok(true)
    }

    pub async fn change_ownership(&self, arg0: Principal) -> Result<Nat, String> {
        let canister = self
            .metadata
            .clone()
            .map(|f| f.metadata.asset_canister)
            .ok_or("Metadata not set".to_string())?;

        let current_user = self
            .metadata
            .clone()
            .map(|f| f.metadata.collection_owner)
            .ok_or("Metadata not set".to_string())?;

        crate::permissions::grant_asset_edit_perms(canister, arg0).await?;

        crate::permissions::revoke_asset_edit_perms(canister, current_user).await?;

        Ok(self.transactions.index().clone())
    }

    pub async fn get_booked_tokens(&self, arg0: Option<Principal>) -> u128 {
        let user = arg0.unwrap_or(caller());

        self.escrow
            .clone()
            .get_booked_tokens()
            .get(&user)
            .cloned()
            .unwrap_or(0)
    }

    pub async fn get_escrow_account(&self) -> Result<GetEscrowAccountRet, String> {
        let escrow_store = self.escrow.clone(); // Assume this retrieves the EscrowStore instance

        if escrow_store.sale_status != SaleStatus::Live {
            return Err("Sale not live.".to_string());
        }

        let principal = ic_cdk::api::id();
        let subaccount = Subaccount::from(&ic_cdk::caller());

        let account_identifier = AccountIdentifier::from_principal(
            principal,
            Some(subaccount),
        );

        Ok(GetEscrowAccountRet {
            account: GetEscrowAccountRetAccount {
                owner: principal,
                subaccount: subaccount.0,
            },
            account_id: account_identifier.to_hex(),
        })
    }

    pub async fn get_metadata(&self) -> Result<GetMetadataRet, String> {
        Ok(self
            .metadata
            .as_ref()
            .map(|f| f.metadata.with_supply(f.total_supply.into()))
            .clone()
            .ok_or("Init args not set".to_string())?)
    }

    pub async fn get_participating_investors(&self) -> Vec<Principal> {
        self.escrow.clone().get_participating_investors()
    }

    pub async fn get_sale_status(&self) -> SaleStatus {
        self.escrow.clone().get_sale_status().clone()
    }

    pub async fn get_total_booked_tokens(&self) -> u128 {
        self.escrow.clone().get_total_booked_tokens().clone()
    }

    pub async fn icrc_61_supported_standards(
        &self,
    ) -> CallResult<(Vec<Icrc61SupportedStandardsRetItem>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc61_supported_standards", ()).await
    }

    pub fn icrc_7_balance_of(&self, arg0: Vec<Icrc7BalanceOfArgItem>) -> Vec<u64> {
        arg0.iter()
            .map(|account| {
                let account_id = TokenState::to_account_id(
                    account.owner.to_text().as_str(),
                    &Some(account.subaccount.clone()),
                );
                self.tokens
                    .owner_to_token_index
                    .get(&account_id)
                    .map_or(0, |tokens| tokens.len() as u64)
            })
            .collect()
    }

    // pub async fn icrc_7_collection_metadata(
    //     &self,
    // ) -> CallResult<(Vec<(String, Icrc7CollectionMetadataRetItem1)>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_collection_metadata", ()).await
    // }
    // pub async fn icrc_7_description(&self) -> CallResult<(Option<String>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_description", ()).await
    // }
    // pub async fn icrc_7_logo(&self) -> CallResult<(Option<String>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_logo", ()).await
    // }
    // pub async fn icrc_7_max_default_take_value(&self) -> CallResult<(Option<candid::Nat>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_max_default_take_value", ()).await
    // }
    // pub async fn icrc_7_max_memo_size(&self) -> CallResult<(Option<candid::Nat>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_max_memo_size", ()).await
    // }
    // pub async fn icrc_7_max_query_batch_size(&self) -> CallResult<(Option<candid::Nat>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_max_query_batch_size", ()).await
    // }
    // pub async fn icrc_7_max_take_value(&self) -> CallResult<(Option<candid::Nat>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_max_take_value", ()).await
    // }
    // pub async fn icrc_7_max_update_batch_size(&self) -> CallResult<(Option<candid::Nat>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_max_update_batch_size", ()).await
    // }
    // pub async fn icrc_7_name(&self) -> CallResult<(String,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_name", ()).await
    // }
    pub fn icrc_7_owner_of(&self, arg0: Vec<u32>) -> Vec<Option<Icrc7OwnerOfRetItemInner>> {
        arg0.into_iter()
            .map(|id| self.tokens.tokens.get(&id)) // Get the token by ID
            .map(|token| {
                token.map(|token| Icrc7OwnerOfRetItemInner {
                    owner: token.owner.principal.clone(),
                    subaccount: token.owner.subaccount.clone(),
                })
            })
            .collect()
    }
    // pub async fn icrc_7_permitted_drift(&self) -> CallResult<(Option<candid::Nat>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_permitted_drift", ()).await
    // }
    // pub async fn icrc_7_supply_cap(&self) -> CallResult<(Option<candid::Nat>,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_supply_cap", ()).await
    // }
    // pub async fn icrc_7_symbol(&self) -> CallResult<(String,)> {
    //     ic_cdk::call(Principal::anonymous(), "icrc7_symbol", ()).await
    // }
    pub fn icrc_7_token_metadata(
        &self,
        arg0: Vec<u32>,
    ) -> Vec<Option<Vec<(String, Icrc7TokenMetadataRetItemInnerItem1)>>> {
        arg0.iter()
            .map(|id| {
                if self.tokens.tokens.contains_key(id) {
                    Some(vec![]) // MetadataResult is empty for existing tokens
                } else {
                    None // Return None for non-existing tokens
                }
            })
            .collect()
    }
    pub fn icrc_7_tokens(&self, prev: Option<u32>, take: Option<u32>) -> Vec<u32> {
        let mut tokens: Vec<_> = self.tokens.tokens.keys().cloned().collect();
        tokens.sort();

        // Convert `prev` to the starting token ID or 0
        let prev_id = prev.unwrap_or(0);

        // Find the index of the `prev_id` in the sorted token list
        let prev_index = if prev_id == 0 {
            -1
        } else {
            tokens
                .iter()
                .position(|&id| id == prev_id)
                .map_or(-1, |idx| idx as isize)
        };

        // Determine the number of tokens to take (default to 5 if `take` is not provided)
        let take_count = take.unwrap_or(5) as usize;

        // Slice the tokens based on the computed index and take count
        tokens
            .into_iter()
            .skip((prev_index + 1) as usize)
            .take(take_count)
            .collect()
    }
    pub fn icrc_7_tokens_of(
        &self,
        account: Icrc7TokensOfArg,
        prev: Option<u32>,
        take: Option<u32>,
    ) -> Vec<u32> {
        let account_id = TokenState::to_account_id(&account.owner.to_text(), &account.subaccount);

        // Get the tokens associated with the account ID
        let tokens: Vec<u32> = self
            .tokens
            .owner_to_token_index
            .get(&account_id)
            .map_or_else(Vec::new, |set| {
                let mut vec: Vec<u32> = set.keys().cloned().collect();
                vec.sort(); // Ensure tokens are sorted
                vec
            });

        // Convert `prev` to the starting token ID or 0
        let prev_id = prev.unwrap_or(0);

        // Find the index of the `prev_id` in the sorted token list
        let prev_index = if prev_id == 0 {
            -1
        } else {
            tokens
                .iter()
                .position(|&id| id == prev_id)
                .map_or(-1, |idx| idx as isize)
        };

        // Determine the number of tokens to take (default to 5 if `take` is not provided)
        let take_count = take.unwrap_or(5) as usize;

        // Slice the tokens based on the computed index and take count
        tokens
            .into_iter()
            .skip((prev_index + 1) as usize)
            .take(take_count)
            .collect()
    }
    pub async fn icrc_7_total_supply(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_total_supply", ()).await
    }

    fn is_subaccounts_eq(a: &Option<Vec<u8>>, b: &Option<Vec<u8>>) -> bool {
        let default_subaccount = vec![0; 32]; // Default subaccount is 32 zero bytes
        let a_str = a.as_ref().unwrap_or(&default_subaccount);
        let b_str = b.as_ref().unwrap_or(&default_subaccount);
        a_str == b_str
    }

    pub fn icrc_7_transfer(
        &mut self,
        args: Vec<Icrc7TransferArgItem>,
    ) -> Vec<Option<Icrc7TransferRetItemInner>> {
        args.into_iter()
            .map(|arg| {
                let token_id = arg.token_id;

                let token = match self.tokens.tokens.get(&token_id) {
                    Some(t) => t,
                    None => {
                        return Some(Icrc7TransferRetItemInner::Err(
                            Icrc7TransferRetItemInnerErr::NonExistingTokenId,
                        ))
                    }
                };

                // Validate token ownership
                if token.owner.principal != caller()
                    || !Self::is_subaccounts_eq(&token.owner.subaccount, &arg.from_subaccount)
                {
                    return Some(Icrc7TransferRetItemInner::Err(
                        Icrc7TransferRetItemInnerErr::Unauthorized,
                    ));
                }

                // Validate recipient
                if caller() == arg.to.owner
                    && !Self::is_subaccounts_eq(&token.owner.subaccount, &arg.to.subaccount)
                {
                    return Some(Icrc7TransferRetItemInner::Err(
                        Icrc7TransferRetItemInnerErr::InvalidRecipient,
                    ));
                }

                self.tokens
                    .transfer(token_id, arg.to.owner, arg.to.subaccount);

                // Return the transaction index as the result
                Some(Icrc7TransferRetItemInner::Ok(self.tokens.counter))
            })
            .collect()
    }
    pub async fn icrc_7_tx_window(&self) -> CallResult<(Option<candid::Nat>,)> {
        ic_cdk::call(Principal::anonymous(), "icrc7_tx_window", ()).await
    }
    pub async fn refund_excess_after_sale(
        &self,
        arg0: Principal,
    ) -> Result<bool, String> {
        self.escrow.refund_from_escrow(&arg0, self.metadata.clone().unwrap().metadata.clone() ).await?;
        Ok(true)
    }

    // Validate collection owner
    pub async fn reject_sale(&self) -> Result<bool, String> {
        // validations::check_collection_owner()?;

        // Check if the sale is live
        if self.escrow.sale_status != SaleStatus::Live {
            return Result::Err("Sale not live.".to_string());
        }


        // Process refunds for all booked tokens
        for (investor_principal, _) in self.escrow.booked_tokens.iter() {
            match self.escrow.refund_from_escrow(investor_principal, self.metadata.as_ref().unwrap().metadata.clone()).await {
                Result::Err(err) => return Result::Err(err),
                _ => {
                STATE.with_borrow_mut(|f| f.escrow.reject_sale_update_invester_booked_tokens(investor_principal));

                }
            }
        }

        // Reject the sale
        STATE.with_borrow_mut(|F| F.escrow.reject_sale());


        Result::Ok(true)
    }


    // pub async fn reject_sale_individual(
    //     &self,
    //     arg0: Principal,
    // ) -> Result<bool, String>{
    //     self.escrow.refund_from_escrow(&arg0, self.metadata.clone().unwrap().metadata.clone() ).await?;
    //     Ok(true)
    // }

    pub async fn update_metadata(
        &self,
        arg0: UpdateMetadataArg,
    ) -> CallResult<(UpdateMetadataRet,)> {
        ic_cdk::call(Principal::anonymous(), "update_metadata", (arg0,)).await
    }
}
