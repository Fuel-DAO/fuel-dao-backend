use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::api::monitoring::MonitoringState;



#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct State {
    pub cars: BTreeMap<u64, Car>,
    pub monitoring: MonitoringState,
    pub controllers: Vec<Principal>,
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
    pub fn get_booking_status_at_give_time_period(&self, start_time: u64, end_time: u64) -> CarStatus {
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
    fn times_overlap(existing_start: u64, existing_end: u64, new_start: u64, new_end: u64) -> bool {
        !(new_end <= existing_start || new_start >= existing_end)
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct  CarAvailability {
  pub   details: CarDetails, 
  pub  available: Option<RentalTransaction>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CarDetails {
    pub id: u64,
    pub make: String,
    pub model: String,
    pub year: u32,
    pub description: String, 
    pub default_image_url: String, 
    pub images: Vec<String>,
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
    pub customer: Option<CustomerDetials>,
    pub start_timestamp: u64, // Unix timestamp
    pub end_timestamp: u64,   // Unix timestamp
    pub total_amount: f64,
    pub payment_status: PaymentStatus,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CustomerDetials {
    pub name: String,
    pub email: String,
    pub country_code: String,
    pub mobile_number: String,
    pub age: u8,
    pub pan: String, 
    pub aadhar: String,
}

impl CustomerDetials {
    pub fn validate_details(&self) -> Result<(), String> {
        if self.name.trim().len() < 3 {return  Err("Invalid Name, please provide a name with more than 4 characters.".into()) ;}
        if self.email.trim().len() < 5 {return  Err("Invalid email, please provide a valid email adress".into()) ;}
        if self.country_code.trim().len() != 2  {return  Err("Invalid country code, please provide a valid country code".into()) ;}
        if self.mobile_number.trim().len() != 10  {return  Err("Invalid mobile number, please provide a 10 digits mobile number".into()) ;}
        if (self.pan.trim().is_empty() || self.pan.trim().len() < 10) && (self.aadhar.trim().is_empty() || self.aadhar.trim().len() != 12)  {return  Err("Invalid documents, please provide a PAN or Aadhar".into()) ;}
        if self.age < 18  {return  Err("Invalid age, age should be atleast 18".into()) ;}
        Ok(())
    }
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


