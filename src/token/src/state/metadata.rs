use candid::{CandidType, Deserialize, Nat, Principal};

use super::models::GetMetadataRet;



#[derive(CandidType, Deserialize, Clone)]
pub struct Metadata {
    pub weight: f64,
    pub drive_type: String,
    pub purchase_price: u128,
    pub token: Principal,
    pub documents: Vec<(String, String)>,
    pub supply_cap: u128,
    pub displays: String,
    pub seating: String,
    pub cargo: f64,
    pub logo: String,
    pub name: String,
    pub overall_height: f64,
    pub description: String,
    pub overall_width: f64,
    pub track_front: f64,
    pub collection_owner: Principal,
    pub asset_canister: Principal,
    pub ground_clearance: f64,
    pub key_features: Vec<String>,
    pub range_per_charge: f64,
    pub track_rear: f64,
    pub acceleration: String,
    pub charging_speed: String,
    pub wheels: f64,
    pub brochure_url: String,
    pub index: Principal,
    pub price: f64,
    pub battery: String,
    pub overall_length: f64,
    pub symbol: String,
    pub treasury: Principal,
    pub images: Vec<String>,
}



impl Metadata {
    pub fn with_supply(&self, total_supply: Nat) -> GetMetadataRet {
        GetMetadataRet {
            weight: self.weight,
            drive_type: self.drive_type.clone(),
            purchase_price: self.purchase_price.clone(),
            token: self.token,
            documents: self.documents.clone(),
            supply_cap: self.supply_cap.clone(),
            displays: self.displays.clone(),
            seating: self.seating.clone(),
            cargo: self.cargo,
            logo: self.logo.clone(),
            name: self.name.clone(),
            overall_height: self.overall_height,
            description: self.description.clone(),
            overall_width: self.overall_width,
            track_front: self.track_front,
            collection_owner: self.collection_owner,
            asset_canister: self.asset_canister,
            ground_clearance: self.ground_clearance,
            key_features: self.key_features.clone(),
            range_per_charge: self.range_per_charge,
            track_rear: self.track_rear,
            acceleration: self.acceleration.clone(),
            charging_speed: self.charging_speed.clone(),
            wheels: self.wheels,
            brochure_url: self.brochure_url.clone(),
            index: self.index,
            price: self.price.clone(),
            battery: self.battery.clone(),
            overall_length: self.overall_length,
            total_supply,
            symbol: self.symbol.clone(),
            treasury: self.treasury,
            images: self.images.clone(),
        }
    }

    pub fn update(&mut self, args: UpdateMetadataArgs) -> Result<u128, String> {
        if let Some(weight) = args.weight {
            self.weight = weight;
        }
        if let Some(drive_type) = args.drive_type {
            self.drive_type = drive_type;
        }
        if let Some(purchase_price) = args.purchase_price {
            self.purchase_price = purchase_price;
        }
        if let Some(token) = args.token {
            self.token = token;
        }
        if let Some(documents) = args.documents {
            self.documents = documents;
        }
        if let Some(supply_cap) = args.supply_cap {
            self.supply_cap = supply_cap;
        }
        if let Some(displays) = args.displays {
            self.displays = displays;
        }
        if let Some(seating) = args.seating {
            self.seating = seating;
        }
        if let Some(cargo) = args.cargo {
            self.cargo = cargo;
        }
        if let Some(logo) = args.logo {
            self.logo = logo;
        }
        if let Some(name) = args.name {
            self.name = name;
        }
        if let Some(overall_height) = args.overall_height {
            self.overall_height = overall_height;
        }
        if let Some(description) = args.description {
            self.description = description;
        }
        if let Some(overall_width) = args.overall_width {
            self.overall_width = overall_width;
        }
        if let Some(track_front) = args.track_front {
            self.track_front = track_front;
        }
        if let Some(asset_canister) = args.asset_canister {
            self.asset_canister = asset_canister;
        }
        if let Some(ground_clearance) = args.ground_clearance {
            self.ground_clearance = ground_clearance;
        }
        if let Some(key_features) = args.key_features {
            self.key_features = key_features;
        }
        if let Some(range_per_charge) = args.range_per_charge {
            self.range_per_charge = range_per_charge;
        }
        if let Some(track_rear) = args.track_rear {
            self.track_rear = track_rear;
        }
        if let Some(acceleration) = args.acceleration {
            self.acceleration = acceleration;
        }
        if let Some(charging_speed) = args.charging_speed {
            self.charging_speed = charging_speed;
        }
        if let Some(wheels) = args.wheels {
            self.wheels = wheels;
        }
        if let Some(brochure_url) = args.brochure_url {
            self.brochure_url = brochure_url;
        }
        if let Some(index) = args.index {
            self.index = index;
        }
        if let Some(price) = args.price {
            self.price = price;
        }
        if let Some(battery) = args.battery {
            self.battery = battery;
        }
        if let Some(overall_length) = args.overall_length {
            self.overall_length = overall_length;
        }
        if let Some(symbol) = args.symbol {
            self.symbol = symbol;
        }
        if let Some(treasury) = args.treasury {
            self.treasury = treasury;
        }
        if let Some(images) = args.images {
            self.images = images;
        }

        // Return success with an updated supply cap
        Ok(self.supply_cap)
    }
}


#[derive(CandidType, Deserialize)]
pub struct UpdateMetadataArgs {
    pub weight: Option<f64>,
    pub drive_type: Option<String>,
    pub purchase_price: Option<u128>,
    pub token: Option<Principal>,
    pub documents: Option<Vec<(String, String)>>,
    pub supply_cap: Option<u128>,
    pub displays: Option<String>,
    pub seating: Option<String>,
    pub cargo: Option<f64>,
    pub logo: Option<String>,
    pub name: Option<String>,
    pub overall_height: Option<f64>,
    pub description: Option<String>,
    pub overall_width: Option<f64>,
    pub track_front: Option<f64>,
    pub asset_canister: Option<Principal>,
    pub ground_clearance: Option<f64>,
    pub key_features: Option<Vec<String>>,
    pub range_per_charge: Option<f64>,
    pub track_rear: Option<f64>,
    pub acceleration: Option<String>,
    pub charging_speed: Option<String>,
    pub wheels: Option<f64>,
    pub brochure_url: Option<String>,
    pub index: Option<Principal>,
    pub price: Option<f64>,
    pub battery: Option<String>,
    pub overall_length: Option<f64>,
    pub symbol: Option<String>,
    pub treasury: Option<Principal>,
    pub images: Option<Vec<String>>,
}