use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{
    btreemap::BTreeMap,
    memory_manager::{MemoryId, MemoryManager, VirtualMemory}, storable::Bound, DefaultMemoryImpl, Storable
};
use std::{borrow::Cow, cell::RefCell};
use candid::Deserialize;
use serde::Serialize;

use crate::{api::monitoring::MonitoringState, Car, DistanceTravelled, RentalTransaction};

#[derive(Deserialize, Serialize)]
pub struct State {
    #[serde(skip, default = "default_car_details")]
    pub cars: BTreeMap<u64, Car, Memory>,
    pub monitoring: MonitoringState,
    // #[serde(skip, default = "default_controller_details")]
    pub controllers: Vec<Principal>,
    #[serde(skip, default = "default_rental_tx_details")]
    pub unpaid_bookings: BTreeMap<u64, RentalTransaction, Memory>,
    #[serde(skip, default = "default_distance_details")]
    pub car_travel_details: BTreeMap<u64, DistanceTravelled, Memory>,
}


// A memory for upgrades, where data from the heap can be serialized/deserialized.
const UPGRADES: MemoryId = MemoryId::new(1);

// A memory for the StableBTreeMap we're using. A new memory should be created for
// every additional stable structure.

const STABLE_CARS_MEMORY: MemoryId = MemoryId::new(2);
const STABLE_RENTAL_TX_MEMORY: MemoryId = MemoryId::new(3);
const STABLE_DISTANCE_MEMORY: MemoryId = MemoryId::new(4);
const STABLE_CONTROLLER_MEMORY: MemoryId = MemoryId::new(5);

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

// Get the memory for the stable structure.
pub fn get_stable_cars_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow_mut().get(STABLE_CARS_MEMORY))
}


pub fn default_car_details(
) -> ic_stable_structures::btreemap::BTreeMap<u64, Car, Memory> {
    ic_stable_structures::btreemap::BTreeMap::init(get_stable_cars_memory())
}

pub fn get_rental_tx_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow_mut().get(STABLE_RENTAL_TX_MEMORY))
}


pub fn default_rental_tx_details(
) -> ic_stable_structures::btreemap::BTreeMap<u64, RentalTransaction, Memory> {
    ic_stable_structures::btreemap::BTreeMap::init(get_rental_tx_memory())
}

 fn get_controllers_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow_mut().get(STABLE_CONTROLLER_MEMORY))
}


 fn default_controller_details(
) -> ic_stable_structures::vec::Vec<Principal, Memory> {
    ic_stable_structures::vec::Vec::init(get_controllers_memory()).expect("failed to get Default controllers")
}

pub fn get_distance_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow_mut().get(STABLE_DISTANCE_MEMORY))
}


pub fn default_distance_details(
) -> ic_stable_structures::btreemap::BTreeMap<u64, DistanceTravelled, Memory> {
    ic_stable_structures::btreemap::BTreeMap::init(get_distance_memory())
}

// const MAX_STORABLE_BOUND_DETAILS_VALUE_SIZE: u32 = 100 as u32;

impl Storable for Car {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for RentalTransaction {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for DistanceTravelled {
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
            cars: default_car_details(),
            monitoring: MonitoringState::default(),
            controllers: Vec::default(),
            unpaid_bookings: default_rental_tx_details(),
            car_travel_details: default_distance_details(),
        }
    }

}