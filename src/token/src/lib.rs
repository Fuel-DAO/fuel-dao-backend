mod permissions;
mod ports;
mod state;
mod validations;
use crate::state::escrow::SaleStatus;
use crate::state::icrc7::ICRC7MetadataQueryResult;
use crate::state::metadata::*;
use crate::state::supported_standards::SupportedStandard;
use candid::Nat;
use candid::Principal;
use ic_cdk::storage;
use ic_cdk_macros::*;
use state::metadata::Metadata;
use state::models::*;
use state::MetaDataState;
use state::State;
use std::cell::RefCell;
use icrc_ledger_types::icrc1::transfer::BlockIndex;
use icrc_ledger_types::icrc1::transfer::TransferArg;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(Default::default());
}

#[ic_cdk_macros::init]
fn init(base: CanisterArgs) {
    match base {
        CanisterArgs::Upgrade => {}
        CanisterArgs::Init { metadata } => init_hook(metadata),
    }
}

fn init_hook(meta: Metadata) {
    STATE.with_borrow_mut(|state| {
        *state = State {
            metadata: Some(MetaDataState {
                metadata: meta,
                total_supply: 0,
            }),
            ..Default::default()
        };
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        storage::stable_save((State {
            metadata: state.borrow().metadata.clone(),
            escrow: state.borrow().escrow.clone(),
            transactions: state.borrow().transactions.clone(),
            tokens: state.borrow().tokens.clone(),
        },))
        .unwrap()
    });
}

#[post_upgrade]
fn post_upgrade(upgrade: CanisterArgs) {
    let state: Result<(State,), String> = storage::stable_restore();
    match state {
        Ok(state) => {
            STATE.with(|s| {
                *s.borrow_mut() = state.0;
            });
        }
        Err(e) => {
            println!("Failed to do post upgrade {e}");
        }
    }
}

ic_cdk_macros::export_candid!();
