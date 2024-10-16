
use candid::Nat;
use ic_cdk::{api::management_canister::http_request::{ CanisterHttpRequestArgument, HttpHeader, HttpMethod}, println};
use ic_cdk_macros::{update, query};
use serde_json::json;
use base64::{ engine::general_purpose, Engine};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use crate::controller::is_controller;

use crate::{RentalTransaction, STATE};


#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct MailState {
    access_token: String, 
    refresh_token: String, 
    client_secret: String, 
    client_id: String,
}

#[update(guard="is_controller")]
pub fn set_mail_state(mail: MailState) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.mail_state = Some(mail);
    })
}

#[query(guard="is_controller")]
pub fn is_mail_state_set() -> bool {
    STATE.with(|state| {
        let  state = state.borrow();
        state.mail_state.is_some()
    })
}


#[derive(Deserialize)]
 struct AccessToken {
     access_token: String,
}

impl MailState {
    async fn update_access_token(&mut self) -> Result<MailState, String> {
        let url  = "https://oauth2.googleapis.com/token";
        let payload = json!({
            "client_id": &self.client_id, 
            "client_secret": &self.client_secret, 
            "refresh_token": &self.refresh_token,
            "grant_type": "refresh_token"
        });
        let request = CanisterHttpRequestArgument {
            method: HttpMethod::POST ,
            url: url.to_string(),
            headers: Vec::new(),
            body: Some(serde_json::to_vec(&payload,).unwrap()),
            max_response_bytes: None,
            transform: None
        };
        
        let cycles = 230_949_972_000;

        // Send the HTTP request to the Gmail API
        match ic_cdk::api::management_canister::http_request::http_request(request, cycles).await {
            Ok((response,)) => {
                if response.status == Nat::from(200 as u32) {
                    let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");

                let access_token: AccessToken = serde_json::from_str(&str_body).expect("Failed to parse json");
                self.access_token = access_token.access_token;
                Ok(self.clone())
                } else {

                    let res = format!("Error: {:?}", response);

                    println!("{res}");
                    
                    Err(res)
                }
            },
            Err((code, message)) => Err(format!("HTTP error {:?}: {}", code, message)),
        }
    }
}


#[update]
pub async  fn refresh_token() -> Result<(), String> {
    let state = STATE.with(|f| f.borrow().mail_state.clone() ) ;

            match  state {

                Some( mut mail) => {
                     match mail.update_access_token().await {
                        Ok(state) => {STATE.with_borrow_mut(|f|  f.mail_state = Some(state) );
                         Ok(())
                        },
                        Err( e) => Err(e) 
                     }

                } , 
                None => Err(String::from("Mail State needs to be set"))
            }
}

pub async fn send_email_gmail(reservation: RentalTransaction) -> Result<(), String> {
    let mail_state = STATE.with(|state| state.borrow().mail_state.clone());

    let username = &reservation.customer.as_ref().unwrap().name;
    let to = &reservation.customer.as_ref().unwrap().email;
    let cc = "bookings@fueldao.io";
    let booking_id = format!("{}-{}", reservation.car_id, &reservation.booking_id);
    let start_date = crate::utils::format_datetime(reservation.start_timestamp);
    let end_date = crate::utils::format_datetime(reservation.end_timestamp);


    match mail_state {
        Some( state) => {
            let access_token = state.access_token;
    let subject = "Booking Confirmed with FuelDao";
    let body = format!("Hey {username},\n\nThank you for choosing FuelDAO! This is a confirmation email of your booking ID {booking_id} with us from {start_date} IST to {end_date} IST.\n\nWatch this space for more details regarding your vehicle details and other information to make it a smooth experience.\n\nRegards\nTeam FuelDao");
    let url = "https://www.googleapis.com/gmail/v1/users/me/messages/send";

    // Create the email message
    let email_raw = format!(
        "To: {}\r\nCc: {}\r\nSubject: {}\r\n\r\n{}",
        to,cc, subject, body
    );
    let encoded_message = general_purpose::STANDARD.encode(email_raw) /* encode(email_raw) */; // Base64 encode the email

    let payload = json!({
        "raw": encoded_message
    });

    let cycles = 230_949_972_000;


    // Create the HTTP request to the Gmail API
    let request = CanisterHttpRequestArgument {
        method: HttpMethod::POST ,
        url: url.to_string(),
        headers: vec![
            HttpHeader {
                name: "Authorization".to_string(), 
                value: format!("Bearer {}", access_token)
            },
            HttpHeader {
                name: "Content-Type".to_string(), 
                value: "application/json".to_string()
            }
           
        ],
        body: Some(serde_json::to_vec(&payload,).unwrap()),
        max_response_bytes: None,
        transform: None
    };

    // Send the HTTP request to the Gmail API
    match ic_cdk::api::management_canister::http_request::http_request(request, cycles).await {
        Ok((response,)) => {
            if response.status == Nat::from(200 as u32) {
                // Ok(response)
                Ok(())
            } else {
                Err(format!("Error: {:?}", response))
            }
        },
        Err((code, message)) => Err(format!("HTTP error {:?}: {}", code, message)),
    }
        }, 
        None => {
            println!("Mail State needs to be set");
              return  Err(String::from("Mail State needs to be set"));
        }
    }

}
