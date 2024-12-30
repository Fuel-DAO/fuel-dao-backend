use ciborium::de;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::Memory;
use super::memory::{self};

use crate::{life_cycle::memory::State, STATE};

#[post_upgrade]
fn post_upgrade() {
    restore_data_from_stable_memory();
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
