use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::STATE;


#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct State {
    pub cars: BTreeMap<u64, Car>,
    pub monitoring: BTreeMap<Principal, Vec<EventMoniter>>
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventMoniter {
    SearchInitiate { current_timestamp: u128, user_principal: Principal}, 
    SelectedCar{car_id: u64 ,current_timestamp: u128, user_principal: Principal }, 
    CarCheckout{car_id: u64, current_timestamp: u128, user_principal: Principal},
}

impl EventMoniter {
    /// TODO: Implement other events
    pub fn search_all_cars() {
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            let user = ic_cdk::caller();
            let event = Self::SearchInitiate { current_timestamp: time() as u128, user_principal: user };
             if  let Some(monitering) = state.monitoring.get_mut(&user) {
                    monitering.push(event);
            } else {
                state.monitoring.insert(user, vec![event]);
            }
        });
    } 
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Car {
    pub id: u64,
    pub details: CarDetails,
    pub bookings: Vec<RentalTransaction>, 
    // pub photos: Vec<String>
    // pub monitoring: Vec<EventMoniter>
}

impl Car {
    pub fn get_booking_status_at_give_time_period(&self, start_time: u128, end_time: u128) -> CarStatus {
    ///  TODO:  VERIFY THIS FUNCTION
    //    if self.details.status == CarStatus::Unavailable || self.details.status == CarStatus::UnderMaintenance {
    //        return   self.details.status.clone();
    //    } 
       for booking in &self.bookings {
        if Self::times_overlap(
            booking.start_timestamp, 
            booking.end_timestamp, 
            start_time, 
            end_time
        ) {
          return   CarStatus::Unavailable;
        }
    }
    self.details.status.clone() 


    }
    fn times_overlap(existing_start: u128, existing_end: u128, new_start: u128, new_end: u128) -> bool {
        !(new_end <= existing_start || new_start >= existing_end)
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CarDetails {
    pub make: String,
    pub model: String,
    pub year: u32,
    // pub default_image_url: String,
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

#[derive(CandidType, Deserialize, Serialize, Clone, PartialEq, Debug)]
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
    pub start_timestamp: u128, // Unix timestamp
    pub end_timestamp: u128,   // Unix timestamp
    pub total_amount: f64,
    pub payment_status: PaymentStatus,
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


