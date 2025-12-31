use axum::{
    Json, Router,
    routing::{delete, get, patch, post, put},
};
use std::collections::HashMap;

use serde_json::Value;
use tokio::task::JoinHandle;
use tracing::info;

use crate::{
    business::{
        app_state::AppState,
        server::{
            RequestIdentifier, connection_establisher::ConnectionEstablisher,
        },
    },
    logging::http_trace::HttpTracingMiddleware,
    model::{
        http_method::HttpMethod,
        request::registration_request::RegistrationRequest,
    },
};

pub struct Server {
    connection: JoinHandle<()>,
    port: String,
    data: HashMap<RequestIdentifier, Value>,
}

impl Server {
    pub fn create<T: ConnectionEstablisher>(
        app_state: &AppState<T>,
        registration_request: RegistrationRequest,
    ) -> Server {
        info!(
            "Registering route [{} (@{})] {}.",
            registration_request.method.to_string(),
            &registration_request.port,
            &registration_request.path,
        );

        let RegistrationRequest {
            port,
            path,
            method,
            response,
        } = registration_request;

        let request_identifier =
            RequestIdentifier::new(path.to_string(), method);

        let mut data = app_state
            .remove_server(&port)
            .map(|server| server.data)
            .unwrap_or(HashMap::new());
        data.insert(request_identifier, response);

        let connection =
            Server::create_connection(app_state, port.clone(), &data);

        Server {
            connection,
            port,
            data,
        }
    }

    fn create_connection<T: ConnectionEstablisher>(
        app_state: &AppState<T>,
        port: String,
        data: &HashMap<RequestIdentifier, Value>,
    ) -> JoinHandle<()> {
        let router = Server::create_router(port.clone(), data);
        app_state.create_connection(port, router)
    }

    fn create_router(
        port: String,
        data: &HashMap<RequestIdentifier, Value>,
    ) -> Router {
        let mut router = Router::new();

        for (request_identifier, response) in data {
            let response = Json(response.clone());

            let method_router = match &request_identifier.method {
                HttpMethod::Get => get(async || response),
                HttpMethod::Post => post(async || response),
                HttpMethod::Put => put(async || response),
                HttpMethod::Patch => patch(async || response),
                HttpMethod::Delete => delete(async || response),
            };

            router = router.route(&request_identifier.path, method_router)
        }

        router.with_http_tracing(port)
    }

    pub fn disconnect(&self) {
        self.connection.abort();
    }

    pub fn get_registration_info(&self) -> Vec<String> {
        info!(
            "Collection information about registrations at server on port {}.",
            &self.port
        );

        let mut registrations = vec![];

        for (identifier, _) in &self.data {
            registrations.push(format!(
                "[{}] {}",
                identifier.method.to_string(),
                &identifier.path
            ));
        }

        registrations
    }
}
