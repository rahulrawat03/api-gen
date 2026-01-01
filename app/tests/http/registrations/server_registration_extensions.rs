use api_gen::model::{
    http_method::HttpMethod,
    response::server_registration::{Registration, ServerRegistration},
};
use serde_json::Value;

pub(super) trait ServerRegistrationExtensions {
    fn find(
        &self,
        port: &str,
        method: HttpMethod,
        path: &str,
        response: Value,
    ) -> Option<&Registration>;
}

impl ServerRegistrationExtensions for Vec<ServerRegistration> {
    fn find(
        &self,
        port: &str,
        method: HttpMethod,
        path: &str,
        response: Value,
    ) -> Option<&Registration> {
        self.iter()
            .filter_map(|sr| {
                if sr.port == port {
                    Some(&sr.registrations)
                } else {
                    None
                }
            })
            .flatten()
            .filter(|r| {
                r.method == method && r.path == path && r.response == response
            })
            .last()
    }
}
