use std::{collections::HashMap, sync::RwLock};

use crate::{business::server::Server, util::lock::safe_write};

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
        safe_write(&self.servers, |mut guard| {
            guard.insert(port.to_string(), server);
        });
    }

    pub fn remove_server(&self, port: &str) -> Option<Server> {
        let server = safe_write(&self.servers, |mut guard| {
            guard.remove(port).map(|server| {
                server.stop();
                server
            })
        });

        server.and_then(|server| server)
    }
}
