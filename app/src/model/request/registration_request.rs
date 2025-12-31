use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::http_method::HttpMethod;

#[derive(Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub port: String,
    pub path: String,
    pub method: HttpMethod,
    pub response: Value,
}
