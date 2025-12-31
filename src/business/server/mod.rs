use crate::model::http_method::HttpMethod;

pub mod connection_establisher;
pub mod server;

#[derive(PartialEq, Eq, Hash)]
struct RequestIdentifier {
    pub path: String,
    pub method: HttpMethod,
}

impl RequestIdentifier {
    fn new(path: String, method: HttpMethod) -> Self {
        Self { path, method }
    }
}
