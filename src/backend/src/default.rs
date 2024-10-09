
pub const DEFAULT_CAR_ID: u64 = 0; 
const DEFAULT_IMAGE_URL : &str = "https://imgd.aeplcdn.com/664x374/n/cw/ec/130583/hector-exterior-right-front-three-quarter-73.jpeg?isig=0&q=80";
use ic_cdk_macros::query;

use crate::{Car, CarDetails, CarStatus, CarType, FuelType, Location, TransmissionType, STATE};

impl Default for CarDetails {
    fn default() -> Self {
        CarDetails {
            id: DEFAULT_CAR_ID,
            make: "MG".to_string(),
            model: "ZLX EV".to_string(),
            year: 2024,
            default_image_url: DEFAULT_IMAGE_URL.into(),
            description: "THE NEXT-GEN MG HECTOR – A CAR THAT PAMPERS YOU. When MG Hector was first launched in India, it created quite a stir with its advanced connected technologies".into(),
            car_type: CarType::Sedan,
            price_per_day: 50.0,
            current_price_per_day: 50.0,
            capacity: 4,
            images: Vec::new(),
            status: CarStatus::Available,
            // last_serviced: Some(Utc::now().timestamp() as u64),
            mileage: Some(300),
            fuel_type: FuelType::Petrol,
            transmission_type: TransmissionType::Automatic,
            color: Some("White".to_string()),
            pickup_location: Some(Location {
                address: "Default Pickup Location".to_string(),
                latitude: 0.0,
                longitude: 0.0,
            }),
            dropoff_location: Some(Location {
                address: "Default Dropoff Location".to_string(),
                latitude: 0.0,
                longitude: 0.0,
            }),
        }
    }
}

#[query]
fn get_default_car() -> Option<Car> {
    STATE.with(|state| {
        let state = state.borrow();
        state.cars.get(&DEFAULT_CAR_ID).cloned()
    })
}
