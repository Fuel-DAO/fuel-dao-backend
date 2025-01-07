use std::collections::BTreeMap;

use candid::CandidType;
use ic_cdk_macros::update;

use crate::{models::CarDetails, STATE};
use crate::{is_controller, CarStatus, CarType, FuelType, Location, TransmissionType};


#[update(guard = "is_controller")]
fn add_car(car: CarDetails) -> u64 {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
         let id = state.cars.last_key_value().map_or(1, |f| f.0 + 1);
        state.cars.insert(id, crate::Car {check: Some(true),  id, details: CarDetails { id, ..car }, bookings: BTreeMap::new(), /* monitoring: Vec::new()  */});
        id
    })
}

#[update(guard = "is_controller")]
fn update_car(id: u64, car: CarDetails) {
    STATE.with(|state| {
        let  state = &mut state.borrow_mut().cars.get(&id);
        state.as_mut().map(|f| {
            f.details = car;
            f
        } );
        // let bookings = state.cars.get(&id).map_or( Vec::new() ,|f| f.bookings.clone());
        // state.cars.insert(id, crate::Car { id: id, details: car, bookings });
    });
}


#[derive(CandidType, candid::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UpdateCarDetails {
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<u32>,
    pub description: Option<String>,
    pub default_image_url: Option<String>,
    pub images: Option<Vec<String>>,
    pub car_type: Option<CarType>,
    pub current_price_per_day: Option<f64>,
    pub price_per_day: Option<f64>,
    pub status: Option<CarStatus>,
    pub capacity: Option<u8>,
    pub mileage: Option<u32>,
    pub fuel_type: Option<FuelType>,
    pub transmission_type: Option<TransmissionType>,
    pub color: Option<String>,
    pub pickup_location: Option<Location>,
    pub dropoff_location: Option<Location>,
}


#[update(guard = "is_controller")]
fn update_car_details_by_id(id: u64, updates: UpdateCarDetails) {
    STATE.with(|state| {
        let  mut state = state.borrow_mut();
        if let Some(car) = &mut state.cars.get(&id) {
            let details = &mut car.details;

            if let Some(make) = updates.make {
                details.make = make;
            }
            if let Some(model) = updates.model {
                details.model = model;
            }
            if let Some(year) = updates.year {
                details.year = year;
            }
            if let Some(description) = updates.description {
                details.description = description;
            }
            if let Some(default_image_url) = updates.default_image_url {
                details.default_image_url = default_image_url;
            }
            if let Some(images) = updates.images {
                details.images = images;
            }
            if let Some(car_type) = updates.car_type {
                details.car_type = car_type;
            }
            if let Some(current_price_per_day) = updates.current_price_per_day {
                details.current_price_per_day = current_price_per_day;
            }
            if let Some(price_per_day) = updates.price_per_day {
                details.price_per_day = price_per_day;
            }
            if let Some(status) = updates.status {
                details.status = status;
            }
            if let Some(capacity) = updates.capacity {
                details.capacity = capacity;
            }
            if let Some(mileage) = updates.mileage {
                details.mileage = Some(mileage);
            }
            if let Some(fuel_type) = updates.fuel_type {
                details.fuel_type = fuel_type;
            }
            if let Some(transmission_type) = updates.transmission_type {
                details.transmission_type = transmission_type;
            }
            if let Some(color) = updates.color {
                details.color = Some(color);
            }
            if let Some(pickup_location) = updates.pickup_location {
                details.pickup_location = Some(pickup_location);
            }
            if let Some(dropoff_location) = updates.dropoff_location {
                details.dropoff_location = Some(dropoff_location);
            }
            car.details = details.clone();
            state.cars.insert(id, car.clone());
        }
    });
}
