use ciborium::de;
use ic_cdk::storage;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::Memory;
use super::memory::{self, TempState};

use crate::{life_cycle::memory::State, STATE};

#[post_upgrade]
fn post_upgrade() {
    let state: Result<(TempState,), String> = storage::stable_restore();
    ic_cdk::println!("Post upgrade called {:?}", state);
    match state {
        Ok(state) => {
            STATE.with(|s| {
                for value in state.0.car_travel_details.iter() {
                    s.borrow_mut().car_travel_details.insert(*value.0, value.1.clone());
                }
                for value in state.0.unpaid_bookings.iter() {
                    s.borrow_mut().unpaid_bookings.insert(*value.0, value.1.clone());
                }
                for value in state.0.car_travel_details.iter() {
                    s.borrow_mut().car_travel_details.insert(*value.0, value.1.clone());
                }
                for value in state.0.controllers.iter() {
                    s.borrow_mut().controllers.push(*value);
                }
                s.borrow_mut().monitoring = state.0.monitoring.clone();

            });
            // init_hook();
        }
        Err(e) => {
            println!("Failed to do post upgrade {e}");
        }
    }
    // restore_data_from_stable_memory();
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
