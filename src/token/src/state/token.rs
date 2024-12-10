
use std::collections::BTreeMap;

use candid::{CandidType, Deserialize, Principal,};
use serde::Serialize;
use crate::STATE;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct TokenState {
   pub counter: u32,
   pub tokens: TokenStoreType,
   pub owner_to_token_index: OwnerToTokenIndexType,
}
#[derive(Clone, Serialize, Deserialize, Debug, CandidType)]
pub struct TokenType {
    pub owner: Owner,
}

#[derive(Clone, Serialize, Deserialize, Debug, CandidType)]
pub struct Owner {
    pub principal: Principal,
    pub subaccount: Option<Vec<u8>>,
}
type TokenStoreType = BTreeMap<u32, TokenType>;
type OwnerToTokenIndexType = BTreeMap<String, BTreeMap<u32, bool>>;


impl TokenState {

    // Utility function to derive account ID
pub fn to_account_id(principal: &str, subaccount: &Option<Vec<u8>>) -> String {
    // Generate account ID based on principal and optional subaccount
    match subaccount {
        Some(sub) => format!("{}-{:?}", principal, sub),
        None => principal.to_string(),
    }
}

    pub fn new() -> Self {
        Self {
            counter: 1,
            tokens: BTreeMap::new(),
            owner_to_token_index: BTreeMap::new(),
        }
    }

    pub fn tokens(&self) -> &TokenStoreType {
        &self.tokens
    }

    pub fn owner_to_token_index(&self) -> &OwnerToTokenIndexType {
        &self.owner_to_token_index
    }

    pub fn mint(&mut self, principal: Principal, subaccount: Option<Vec<u8>>) -> u32 {
        let account_id = Self::to_account_id(&principal.to_text(), &subaccount);
        let token_id = self.counter;
        self.counter += 1;

        let user_token_index = self
            .owner_to_token_index
            .entry(account_id.clone())
            .or_insert_with(BTreeMap::new);

        self.tokens.insert(
            token_id,
            TokenType {
                owner: Owner {
                    principal: principal,
                    subaccount: subaccount.clone(),
                },
            },
        );
        user_token_index.insert(token_id, true);

        token_id
    }

    pub fn burn(&mut self, token_id: u32) {
        if let Some(token) = self.tokens.remove(&token_id) {
            let account_id = Self::to_account_id(&token.owner.principal.to_text(), &token.owner.subaccount);

            if let Some(user_token_index) = self.owner_to_token_index.get_mut(&account_id) {
                user_token_index.remove(&token_id);

                if user_token_index.is_empty() {
                    self.owner_to_token_index.remove(&account_id);
                }
            }

            STATE.with_borrow_mut(|F| {F.metadata.as_mut().map(|f| f.decrement_supply());});

        }
    }

    pub fn transfer(&mut self, token_id: u32, principal: Principal, subaccount: Option<Vec<u8>>) {
        if let Some(token) = self.tokens.get_mut(&token_id) {
            let holder_account_id = Self::to_account_id(&token.owner.principal.to_text(), &token.owner.subaccount);
            let receiver_account_id = Self::to_account_id(&principal.to_text(), &subaccount);

            token.owner.principal = principal;
            token.owner.subaccount = subaccount.clone();

            if let Some(holder_token_index) = self.owner_to_token_index.get_mut(&holder_account_id) {
                holder_token_index.remove(&token_id);

                if holder_token_index.is_empty() {
                    self.owner_to_token_index.remove(&holder_account_id);
                }
            }

            self
                .owner_to_token_index
                .entry(receiver_account_id.clone())
                .or_insert_with(BTreeMap::new).insert(token_id, true);
        }
    }
}