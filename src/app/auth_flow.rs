use websocket::Message;
use websocket::sync::Client;
use websocket::ClientBuilder;
use websocket::stream::sync::NetworkStream;
use websocket::ws::dataframe::DataFrame;
use crate::app::auth_flow_models::{AuthWSMessage, AvailableAuthTypes, GetAvailabilityAuthTypesWSMessage, Password};

pub const SERVER_WSS_URL: &str = "wss://launcher.dreamfinity.org/api";

pub fn get_auth_types(client: &mut Client<Box<dyn NetworkStream + Send>>) -> Option<AvailableAuthTypes> {
    let auth_types_msg = GetAvailabilityAuthTypesWSMessage {
        r#type: String::from("getAvailabilityAuth")
    };
    let msg_to_send = match serde_json::to_string::<GetAvailabilityAuthTypesWSMessage>(&auth_types_msg) {
        Ok(msg) => {
            Message::text(msg)
        }
        Err(_) => {
            return None;
        }
    };
    client.send_message::<Message>(&msg_to_send).expect("TODO: panic message");

    match client.recv_message() {
        Ok(message) => {
            let res_bytes = message.take_payload();
            match String::from_utf8(res_bytes) {
                Ok(res) => {
                    Some(serde_json::from_str::<AvailableAuthTypes>(&res).unwrap())
                }
                Err(_) => {
                    None
                }
            }
        }
        Err(_) => {
            None
        }
    }
}

pub fn update_launcher() {}

pub fn authorize() {
    let authMsg = AuthWSMessage {
        r#type: String::from("auth"),
        authType: String::from("CLIENT"),
        login: String::from("user"),
        auth_id: String::from("std"),
        password: Password { password: String::from("pass"), r#type: String::from("plain") },
        getSession: false,
    };
    let mut client
        = match ClientBuilder::new(SERVER_WSS_URL) {
        Ok(mut res) => {
            match res.connect(None) {
                Ok(connect_res) => { connect_res }
                Err(_) => {
                    std::process::exit(-1);
                }
            }
        }
        Err(_) => {
            std::process::exit(-1);
        }
    };
    let available_auth_types = get_auth_types(&mut client);
}