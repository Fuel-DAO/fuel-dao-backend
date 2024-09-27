use candid::Principal;
pub const DEFAULT_CAR_PRINCIPAL: &str = "skk2z-4bfey-fqzzu-resy6-6lj6o-zslme-pd4e7-ehgie-zdnkv-dqurc-nae"; 
use ic_cdk_macros::query;

use crate::{CarDetails, CarStatus, CarType, FuelType, Location, TransmissionType, STATE};

impl Default for CarDetails {
    fn default() -> Self {
        CarDetails {
            make: "Default Make".to_string(),
            model: "Default Model".to_string(),
            year: 2024,
            car_type: CarType::Sedan,
            price_per_day: 50.0,
            status: CarStatus::Available,
            // last_serviced: Some(Utc::now().timestamp() as u64),
            mileage: Some(300),
            is_electric: false,
            fuel_type: FuelType::Petrol,
            transmission_type: TransmissionType::Automatic,
            color: Some("White".to_string()),
            pickup_location: Location {
                address: "Default Pickup Location".to_string(),
                latitude: 0.0,
                longitude: 0.0,
            },
            dropoff_location: Location {
                address: "Default Dropoff Location".to_string(),
                latitude: 0.0,
                longitude: 0.0,
            },
        }
    }
}

#[query]
fn get_default_car() -> Option<CarDetails> {
    let default_car_principal = Principal::from_text(DEFAULT_CAR_PRINCIPAL).expect("Invalid principal format");
    STATE.with(|state| {
        let state = state.borrow();
        state.cars.get(&default_car_principal).cloned()
    })
}
