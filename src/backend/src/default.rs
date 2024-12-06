
pub const DEFAULT_CAR_ID: u64 = 0; 
const DEFAULT_IMAGE_URL : &str = "https://hmh62-uiaaa-aaaai-actxq-cai.icp0.io/public/img/backend/Logo.jpg";
use ic_cdk_macros::query;

use crate::{Car, CarDetails, CarStatus, CarType, FuelType,  TransmissionType, STATE};

impl Default for CarDetails {
    fn default() -> Self {
        CarDetails {
            id: DEFAULT_CAR_ID,
            make: "MG".to_string(),
            model: "ZS EV".to_string(),
            year: 2024,
            default_image_url: DEFAULT_IMAGE_URL.into(),
            description: "The 2024 MG ZS EV: where innovation meets style. This all-electric SUV combines cutting-edge technology with a sleek design, making it the perfect choice for the eco-conscious driver who refuses to compromise on performance or comfort. With a robust electric powertrain, the ZS EV delivers a thrilling drive and impressive range, allowing you to explore your world with confidence. Its spacious interior is crafted for convenience, featuring advanced infotainment options and premium materials that elevate every journey.".into(),
            car_type: CarType::SUV,
            price_per_day: 6000.0,
            current_price_per_day: 4000.0,
            capacity: 5,
            images: vec![
                "https://hmh62-uiaaa-aaaai-actxq-cai.icp0.io/public/img/backend/1.jpeg".to_string(),
                "https://hmh62-uiaaa-aaaai-actxq-cai.icp0.io/public/img/backend/2.jpeg".to_string(),
                "https://hmh62-uiaaa-aaaai-actxq-cai.icp0.io/public/img/backend/3.jpeg".to_string(),
                ],
            status: CarStatus::Available,
            // last_serviced: Some(Utc::now().timestamp() as u64),
            mileage: Some(461),
            fuel_type: FuelType::Electric,
            transmission_type: TransmissionType::Automatic,
            color: Some("https://hmh62-uiaaa-aaaai-actxq-cai.icp0.io/public/img/backend/1.jpeg".to_string()),
            pickup_location: None,
            dropoff_location: None,
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
