// std
use std::collections::HashMap;
use std::{thread, time};
use std::sync::{Arc, Mutex};

// reqwest
use reqwest::blocking::Client;
// websocket
use tungstenite::{connect, Message};
use url::Url;
// tokio
use tokio::runtime::Runtime;
// internal
use crate::ext::extension::Extension;
// serde
use serde_json;

// Add more metadata if needed
pub struct Requester {
    client: Client,
    token: String,
    listeners: HashMap<String, Box<dyn Extension>>
}

pub struct RequesterBuilder {
    data: HashMap<String, String>,
    extensions: Vec<Box<dyn Extension>>
}

impl RequesterBuilder {

    pub fn new() -> RequesterBuilder {
        RequesterBuilder {data: HashMap::new(), extensions: Vec::new()}
    }

    pub fn add_extension<T: 'static + Extension>(&mut self, extension: T) -> &mut Self {
        self.extensions.push(Box::new(extension));
        self
    }

    pub fn add_setting(&mut self, setting: String, value: String) -> &mut Self {
        self.data.insert(setting, value);
        self
    }

    pub fn start(&mut self) -> Requester {
        // TODO: Handle settings
        let mut req = Requester { client: Client::new(), token: self.data.get("token").unwrap().to_owned(), listeners: HashMap::new() };
        req.init();
        req
    }
}

impl Requester {

    pub fn init(&mut self) {
        // TODO: Handle caching URL
        let URL_GET_GATEWAY = "https://discord.com/api/gateway";
        let response = self.client
            .get(URL_GET_GATEWAY)// Create the GET request
            .send()// Send the GET request
            .unwrap();
        let text = &response
            .text()// Read the response as text
            .unwrap();
        let gateway_url: serde_json::Value = serde_json::from_str(text)
            .unwrap();
        let uri = gateway_url
            .get("url")// Finding the 'url' key in the JSON data received
            .unwrap()
            .as_str()// Transforming the value into a &str
            .unwrap();
        let (mut socket, response) = connect(Url::parse(uri)
            .unwrap())
            .expect("Can't connect");
        let msg = socket
            .read_message() // Read the handshake message
            .expect("Error reading message");
        let handshake_data: serde_json::Value = serde_json::from_str(msg
            .to_text()
            .unwrap())
            .unwrap();
        let duration = handshake_data
            .get("d")
            .unwrap()
            .get("heartbeat_interval")
            .unwrap()
            .as_u64()
            .unwrap();
        let s = "null";
        let transferable_socket = Arc::new(Mutex::new(socket));
        {
            let mover = transferable_socket.clone();
            thread::spawn(move || {
                loop {
                    let payload = format!(r#"{{"op": "1", "d": {}}}"#, s);
                    println!("{}", payload);
                    mover.lock().unwrap().write_message(Message::Text(payload)).unwrap();
                    thread::sleep(time::Duration::from_millis(duration));
                }
            });
        }
        loop {
            let msg = transferable_socket.lock().unwrap().read_message().expect("Error reading message");
            println!("Received: {}", msg);
        }
    }
}