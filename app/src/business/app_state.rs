use std::{collections::HashMap, sync::RwLock};

use tracing::info;

use crate::{
    business::server::{
        connection_establisher::ConnectionEstablisher, server::Server,
    },
    model::internal::server_registration::ServerRegistration,
    util::lock::{safe_read, safe_write},
};

pub struct AppState<T: ConnectionEstablisher> {
    servers: RwLock<HashMap<String, Server>>,
    connection_establisher: T,
}

impl<T: ConnectionEstablisher> AppState<T> {
    pub fn new(connection_establisher: T) -> Self {
        Self {
            servers: RwLock::new(HashMap::new()),
            connection_establisher,
        }
    }

    pub fn get_connection_establisher(&self) -> &T {
        &self.connection_establisher
    }

    pub fn add_server(&self, port: &str, server: Server) {
        info!(%port, "Adding server at port {port}.");

        safe_write(&self.servers, |mut guard| {
            guard.insert(port.to_string(), server);
        });
    }

    pub fn remove_server(&self, port: &str) -> Option<Server> {
        info!(%port, "Removing server at port {port}.");

        let server = safe_write(&self.servers, |mut guard| {
            guard.remove(port).map(|server| {
                server.stop();
                server
            })
        });

        server.and_then(|server| server)
    }

    pub fn get_registrations(&self) -> Vec<ServerRegistration> {
        info!("Collecting information about all registrations.");

        let registrations = safe_read(&self.servers, |guard| {
            guard
                .iter()
                .map(|(_, server)| server.get_registrations())
                .collect::<Vec<_>>()
        });

        registrations.unwrap_or(vec![])
    }
}
