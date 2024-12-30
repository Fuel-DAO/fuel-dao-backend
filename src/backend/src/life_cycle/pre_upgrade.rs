use crate::STATE;
use ciborium::ser;
use ic_cdk::api::stable;
use ic_cdk_macros::pre_upgrade;
use ic_stable_structures::writer::Writer;
use super::memory;

#[pre_upgrade]
fn pre_upgrade() {
    let mut state_bytes = vec![];
    STATE.with(|state| {
        ser::into_writer(&*state.borrow(), &mut state_bytes)
    })
    .expect("failed to encode state");

    let len = state_bytes.len() as u32;
    
    if stable::stable_size() == 0 {
        memory::init_memory_manager();
    }
    let mut upgrade_memory = memory::get_upgrades_memory();
    let mut writer = Writer::new(&mut upgrade_memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap();
}