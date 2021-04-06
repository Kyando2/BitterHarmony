// std
use std::collections::HashMap;
// hyper
use hyper::{Body, Method, Request, Uri, Client};
// internal
use crate::Ext::extension::Extension;

// Add more metadata if needed
pub struct Requester {
    core: Client;
    token: String;
}

pub struct RequesterBuilder<T: Extension> {
    data: Hashmap<String, String>;
    extensions: Vec<T>;
}

impl Requester {
    // Creates a new client and starts the connection to the discord WS
    // Local function. Should use method RequesterBuilder.create
    fn new() -> mut Requester {
    }
}