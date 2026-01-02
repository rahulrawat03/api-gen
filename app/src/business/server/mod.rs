use crate::model::http_method::HttpMethod;

pub mod connection_establisher;
pub mod restartable;
pub mod server;

#[derive(PartialEq, Eq, Hash)]
struct RegistrationIdentifier {
    pub path: String,
    pub method: HttpMethod,
}

impl RegistrationIdentifier {
    fn new(path: String, method: HttpMethod) -> Self {
        Self { path, method }
    }
}
