use std::{collections::HashMap, sync::RwLock};

use tracing::info;

use crate::{
    business::server::{Server, ServerFactory},
    model::request::registration_request::RegistrationRequest,
    util::lock::{safe_read, safe_write},
};

pub struct AppState<S: Server, T: ServerFactory<S>> {
    servers: RwLock<HashMap<String, S>>,
    _server_factory: T,
}

impl<S: Server, T: ServerFactory<S>> AppState<S, T> {
    pub fn new(server_factory: T) -> Self {
        Self {
            servers: RwLock::new(HashMap::new()),
            _server_factory: server_factory,
        }
    }

    pub fn create_server(&self, registration_request: RegistrationRequest) -> S {
        T::create(self, registration_request)
    }

    pub fn add_server(&self, port: &str, server: S) {
        info!("Adding server at port {}.", port);

        safe_write(&self.servers, |mut guard| {
            guard.insert(port.to_string(), server);
        });
    }

    pub fn remove_server(&self, port: &str) -> Option<S> {
        info!("Removing server at port {}.", port);

        let server = safe_write(&self.servers, |mut guard| {
            guard.remove(port).map(|server| {
                server.disconnect();
                server
            })
        });

        server.and_then(|server| server)
    }

    pub fn get_registration_info(&self) -> HashMap<String, Vec<String>> {
        info!("Collecting information about all registrations.");

        let registrations = safe_read(&self.servers, |guard| {
            guard
                .iter()
                .map(|(port, server)| (port.to_string(), server.get_registration_info()))
                .collect::<HashMap<_, _>>()
        });

        registrations.unwrap_or(HashMap::new())
    }
}
