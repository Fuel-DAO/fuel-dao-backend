use crate::{Car, CustomerDetials, EventMoniter, RentalTransaction};
use candid::{ decode_one, encode_one,  Decode, Encode, Principal};
use pocket_ic::{PocketIc, WasmResult};
use ic_cdk::api::management_canister::main::CanisterId;
pub const CANISTER_WASM: &[u8] =
    include_bytes!("../../../../target/wasm32-unknown-unknown/release/backend.wasm");


pub struct CanisterSetup {
    env: PocketIc,
    canister_id: CanisterId,
}

impl CanisterSetup {
    pub fn new() -> Self {
        let env = PocketIc::new();
        let canister_id = env.create_canister();
        env.add_cycles(canister_id, u128::MAX);
        env.install_canister(canister_id, CANISTER_WASM.to_vec(), vec![], None);
        Self { env, canister_id }
    }

    pub fn greet(&self, value: &str) -> Option<String> {
        match self
            .env
            .query_call(
                self.canister_id,
                Principal::anonymous(),
                "greet",
                Encode!(&value).unwrap(),
            )
            .expect("failed to get value")
        {
            WasmResult::Reply(bytes) => Decode!(&bytes, Option<String>).unwrap(),
            WasmResult::Reject(e) => {
                panic!("Failed to get value: {:?}", e);
            }
        }
    }

    pub fn list_all_cars(&self) -> Vec<Car> {
        match self
            .env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "list_all_cars",
                encode_one(()).unwrap(),
            )
            .expect("failed to get value")
        {
            WasmResult::Reply(bytes) => Decode!(&bytes, Vec<Car>).unwrap(),
            WasmResult::Reject(e) => {
                panic!("Failed to get value: {:?}", e);
            }
        }
    }

    pub fn reserve_car(&self, car_id: u64, start_timestamp: u64, end_timestamp: u64,customer :CustomerDetials) -> Result<RentalTransaction, String>{
        
        let args = Encode!(&car_id, &start_timestamp, &end_timestamp, &customer).expect("failed to encode args");
        
        match self
            .env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "reserve_car",
                args,
            )
            .expect("failed to Reserve car")
            
        {
            WasmResult::Reply(bytes) => decode_one(&bytes).unwrap() ,
            WasmResult::Reject(e) => {
                println!("Reserve car rejected {e}");
                Err(e.to_string())
            }
        }
    }

    pub fn get_monitoring_events(&self) -> Vec<EventMoniter>{
        match self
            .env
            .query_call(
                self.canister_id,
                Principal::anonymous(),
                "get_monitoring_events",
                encode_one(()).unwrap(),
            )
            .expect("failed to get value")
        {
            WasmResult::Reply(bytes) => Decode!(&bytes, Vec<EventMoniter>).unwrap(),
            WasmResult::Reject(e) => {
                panic!("Failed to get value: {:?}", e);
            }
        }
    }

    pub fn get_controllers(&self, caller: Principal) -> Vec<Principal>{
        
        let args = Encode!(&caller).expect("failed to encode args");

        match self
            .env
            .query_call(
                self.canister_id,
                Principal::anonymous(),
                "get_controllers",
                args,
            )
            .expect("failed to get value")
        {
            WasmResult::Reply(bytes) => Decode!(&bytes, Vec<Principal>).unwrap(),
            WasmResult::Reject(e) => {
                panic!("Failed to get value: {:?}", e);
            }
        }
    }
}

impl Default for CanisterSetup {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_greet() {
    let canister = CanisterSetup::default();

    let input = String::from("Prakash");
    
    assert_eq!(canister.greet(&input), Some(format!("Hello, {}!", input)));
    
}

#[test]
fn test_list_all_cars() {

    let canister = CanisterSetup::default();

    let cars = canister.list_all_cars();

    assert_eq!(cars.len(), 1);

}

#[test]
fn test_reserve_car() {

    let canister = CanisterSetup::default();

    let start_time = 1728475879 ;

    let end_time = start_time - 100;

    let car_id = 0;

    let customer = CustomerDetials {
        name: "FuelDaoUser".into(), 
        email: "prakash@fueldao.io".into(),
        country_code: "91".into(),
        mobile_number: "9876543210".into(),
        age: 18, 
        pan: "USERPAN127NO".into(),
        aadhar:"123456789012".into(), 
        caller: Principal::anonymous(),
    };

    let reservation = canister.reserve_car(car_id, start_time, end_time, customer.clone());

    assert_eq!(reservation.is_err(), true);

    let start_time = 1728475879 ;

    let end_time = start_time + 100;

    let reservation = canister.reserve_car(car_id, start_time, end_time, customer.clone());

    assert_eq!(reservation.is_ok(), true);

    let start_time = 1728475879 + 90 ;

    let end_time = start_time + 100;

    let reservation = canister.reserve_car(car_id, start_time, end_time, customer.clone());

    assert_eq!(reservation.is_ok(), false);

    let start_time = 1728475879 + 100 ;

    let end_time = start_time + 100;

    let reservation = canister.reserve_car(car_id, start_time, end_time, customer.clone());

    assert_eq!(reservation.is_ok(), true);


}

#[test]
fn test_monitoring() {

    let canister = CanisterSetup::default();

    let events = canister.get_monitoring_events();

    assert_eq!(events.len(), 0);

    canister.list_all_cars();

    let events = canister.get_monitoring_events();

    assert_eq!(events.len(), 1);

}


#[test]
fn test_controllers() {

    let canister = CanisterSetup::default();

    let controllers = canister.get_controllers(Principal::anonymous());

    assert_eq!(controllers.len(), 0);

}

