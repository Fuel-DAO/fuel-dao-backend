use ciborium::de;
use ic_cdk::storage;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::Memory;
use super::memory::{self};

use crate::{life_cycle::memory::State, state::TempState, STATE};

#[post_upgrade]
fn post_upgrade() {
    post_upgrade_storage();
    // restore_data_from_stable_memory();
}

fn post_upgrade_storage() {
    let state: Result<(TempState, ), String> = storage::stable_restore();
    match state {
        Ok(state) => {
            STATE.with(|s| {
                for value in state.0.collection_requests.iter() {
                    s.borrow_mut().collection_requests.insert(*value.0, value.1.clone());
                }
                
                s.borrow_mut().admins = state.0.admins;
                s.borrow_mut().asset_proxy_canister = state.0.asset_proxy_canister.clone();
                s.borrow_mut().asset_wasm = state.0.asset_wasm.clone();
                s.borrow_mut().token_wasm = state.0.token_wasm.clone();
                  });
        }, Err(e) => {
            println!("Failed to do post upgrade {e}");
        }
    }
}

fn restore_data_from_stable_memory() {
    let heap_data = memory::get_upgrades_memory();
    let mut heap_data_len_bytes = [0; 4];

    if heap_data.size() == 0 {
        // Handle uninitialized memory gracefully
        ic_cdk::println!("No upgrade data found; initializing default state.");
        STATE.with(|canister_data_ref_cell| {
            *canister_data_ref_cell.borrow_mut() = State::default();
        });
        return;
    }

    heap_data.read(0, &mut heap_data_len_bytes);
    let heap_data_len = u32::from_le_bytes(heap_data_len_bytes) as usize;

    let mut canister_data_bytes = vec![0; heap_data_len];
    heap_data.read(4, &mut canister_data_bytes);

    let canister_data =
        de::from_reader(&*canister_data_bytes).expect   ("Failed to deserialize heap data");
    STATE.with(|canister_data_ref_cell| {
        *canister_data_ref_cell.borrow_mut() = canister_data;
    });
}
