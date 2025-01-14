use candid::{ Decode, Encode, Principal};
use ic_stable_structures::{
    btreemap::BTreeMap,
    memory_manager::{MemoryId, MemoryManager, VirtualMemory}, storable::Bound, DefaultMemoryImpl, Storable
};
use std::{borrow::Cow, cell::RefCell};
use candid::Deserialize;
use serde::Serialize;

use crate::CollectionRequestConfig;


#[derive(Deserialize, Serialize)]
pub struct State {
    pub asset_wasm: Option<Vec<u8>>,
    pub token_wasm: Option<Vec<u8>>,
    pub admins:Vec<Principal>,
    #[serde(skip, default = "default_collection_requests")]
    pub collection_requests: BTreeMap<u64,CollectionRequestConfig, Memory>,
    pub asset_proxy_canister: Option<Principal>,

}
// pub struct State {
//     #[serde(skip, default = "default_car_details")]
//     pub cars: BTreeMap<u64, Car, Memory>,
//     pub monitoring: MonitoringState,
//     // #[serde(skip, default = "default_controller_details")]
//     pub controllers: Vec<Principal>,
//     #[serde(skip, default = "default_collection_requests")]
//     pub unpaid_bookings: BTreeMap<u64, RentalTransaction, Memory>,
//     #[serde(skip, default = "default_distance_details")]
//     pub car_travel_details: BTreeMap<u64, DistanceTravelled, Memory>,
// }


// A memory for upgrades, where data from the heap can be serialized/deserialized.
const UPGRADES: MemoryId = MemoryId::new(1);

// A memory for the StableBTreeMap we're using. A new memory should be created for
// every additional stable structure.

const STABLE_COLLECTION_REQUEST_MEMORY: MemoryId = MemoryId::new(2);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;


thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), 1));
}

pub fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow_mut().get(UPGRADES))
}


pub fn init_memory_manager() {
    MEMORY_MANAGER.with(|m| {
        *m.borrow_mut() = MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), 1);
    })
}

pub fn get_collection_requests_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow_mut().get(STABLE_COLLECTION_REQUEST_MEMORY))
}


pub fn default_collection_requests(
) -> ic_stable_structures::btreemap::BTreeMap<u64, CollectionRequestConfig, Memory> {
    ic_stable_structures::btreemap::BTreeMap::init(get_collection_requests_memory())
}

impl Storable for CollectionRequestConfig {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for State {
    fn default() -> Self {
        State {
            asset_wasm: None,
            token_wasm: None,
            admins: Vec::new(),
            collection_requests: default_collection_requests(),
            asset_proxy_canister: None,
           
        }
    }

}