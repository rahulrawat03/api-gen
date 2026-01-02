use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use api_gen::{
    business::server::connection_establisher::ConnectionEstablisher,
    model::error::Error,
    util::lock::{safe_read, safe_write},
};
use axum::Router;
use tokio::task::JoinHandle;

pub struct FakeConnectionEstablisher {
    routers: Arc<RwLock<HashMap<String, Router>>>,
}

impl Clone for FakeConnectionEstablisher {
    fn clone(&self) -> Self {
        Self {
            routers: self.routers.clone(),
        }
    }
}

impl FakeConnectionEstablisher {
    pub fn new() -> Self {
        Self {
            routers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl ConnectionEstablisher for FakeConnectionEstablisher {
    async fn connect(
        &self,
        port: String,
        router: Router,
    ) -> Result<JoinHandle<()>, Error> {
        safe_write(&self.routers, |mut guard| {
            guard.insert(port, router);
        });

        Ok(tokio::spawn(async move {}))
    }
}

impl FakeConnectionEstablisher {
    pub fn get_router(&self, port: &str) -> Router {
        let router = safe_read(&self.routers, |guard| match guard.get(port) {
            Some(router) => router.clone(),
            None => {
                panic!("No server listening on port {}!", port)
            }
        });

        match router {
            Some(router) => router,
            None => panic!(
                "Something went wrong while retrieving server on port {}!",
                port
            ),
        }
    }
}
