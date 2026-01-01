use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::http_method::HttpMethod;

#[derive(Serialize, Deserialize)]
pub struct ServerRegistration {
    pub port: String,
    pub registrations: Vec<Registration>,
}

impl ServerRegistration {
    pub fn new(port: String, registrations: Vec<Registration>) -> Self {
        Self {
            port,
            registrations,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Registration {
    pub method: HttpMethod,
    pub path: String,
    pub response: Value,
}

impl Registration {
    pub fn new(method: HttpMethod, path: String, response: Value) -> Self {
        Self {
            method,
            path,
            response,
        }
    }
}
