use crate::{
    business::app_state::AppState,
    model::{http_method::HttpMethod, request::registration_request::RegistrationRequest},
};

pub mod default_server;

#[derive(PartialEq, Eq, Hash)]
struct RequestIdentifier {
    path: String,
    method: HttpMethod,
}

impl RequestIdentifier {
    fn new(path: String, method: HttpMethod) -> Self {
        Self { path, method }
    }
}

pub trait Server {
    fn disconnect(&self);

    fn get_registration_info(&self) -> Vec<String>;
}

pub trait ServerFactory<S: Server>: Sized {
    fn create(app_state: &AppState<S, Self>, registration_request: RegistrationRequest) -> S;
}
