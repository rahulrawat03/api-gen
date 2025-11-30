use std::{collections::HashMap, sync::RwLock};

use tracing::info;

use crate::{
    business::server::Server,
    util::lock::{safe_read, safe_write},
};

pub struct AppState {
    servers: RwLock<HashMap<String, Server>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            servers: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_server(&self, port: &str, server: Server) {
        info!("Adding server at port {}.", port);

        safe_write(&self.servers, |mut guard| {
            guard.insert(port.to_string(), server);
        });
    }

    pub fn remove_server(&self, port: &str) -> Option<Server> {
        info!("Removing server at port {}.", port);

        let server = safe_write(&self.servers, |mut guard| {
            guard.remove(port).map(|server| {
                server.stop();
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
