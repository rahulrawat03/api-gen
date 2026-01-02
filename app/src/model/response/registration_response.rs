use serde::{Deserialize, Serialize};

use crate::model::{
    internal::server_registration::Registration,
    request::registration_request::RegistrationRequest,
};

#[derive(Serialize, Deserialize)]
pub struct RegistrationResponse {
    pub added: Registration,
    pub removed: Option<Registration>,
}

impl RegistrationResponse {
    pub fn new(
        registration_request: RegistrationRequest,
        removed_registration: Option<Registration>,
    ) -> Self {
        Self {
            added: Registration::new(
                registration_request.method,
                registration_request.path,
                registration_request.response,
            ),
            removed: removed_registration,
        }
    }
}
