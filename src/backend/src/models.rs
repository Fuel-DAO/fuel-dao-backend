use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

// ---- Structs and Enums ----

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct State {
    pub cars: BTreeMap<u64, Car>,
    // pub bookings: BtreeMap<u64, RentalTransaction  >
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Car {
    pub id: u64,
    pub details: CarDetails,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CarDetails {
    pub make: String,
    pub model: String,
    pub year: u32,
    pub car_type: CarType,
    pub current_price_per_day: f64,
    pub price_per_day: f64,
    pub status: CarStatus,
    pub capacity: u8,
    pub mileage: Option<u32>,
    pub fuel_type: FuelType,
    pub transmission_type: TransmissionType,
    pub color: Option<String>,
    pub pickup_location: Option<Location>,
    pub dropoff_location: Option<Location>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Location {
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum FuelType {
    Petrol,
    Diesel,
    Electric,
    Hybrid,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransmissionType {
    Automatic,
    Manual,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum CarType {
    Sedan,
    SUV,
    Truck,
    Coupe,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum CarStatus {
    Available,
    ComingSoon,
    Unavailable,
    UnderMaintenance,
    Reserved {
        reservation_id: Principal,
        reservation_timestamp: u64, // Unix timestamp
        customer_id: Principal,
    },
    OutOfService { reason: String },
    ScheduledForInspection { inspection_timestamp: u64 }, // Unix timestamp
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct RentalTransaction {
    pub car_principal_id: u64,
    pub customer_principal_id: Principal,
    pub customer_name: String,
    pub start_timestamp: u64, // Unix timestamp
    pub end_timestamp: u64,   // Unix timestamp
    pub total_amount: f64,
    pub payment_status: PaymentStatus,
    pub reservation_id: Option<Principal>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Customer {
    pub principal: Principal,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub id_type: Option<IdType>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum IdType {
    Aadhar(String),
    PAN(String),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum PaymentStatus {
    Paid,
    Unpaid,
}


