use std::collections::HashMap;

use axum::{
    Json, Router,
    routing::{delete, get, patch, post, put},
    serve,
};
use serde_json::Value;
use tokio::{net::TcpListener, task::JoinHandle};
use tracing::info;

use crate::{
    business::{
        app_state::AppState,
        server::{RequestIdentifier, Server, ServerFactory},
    },
    logging::http_trace::HttpTracingMiddleware,
    model::{http_method::HttpMethod, request::registration_request::RegistrationRequest},
};

pub struct DefaultServer {
    connection: JoinHandle<()>,
    port: String,
    data: HashMap<RequestIdentifier, Value>,
}

impl Server for DefaultServer {
    fn disconnect(&self) {
        self.connection.abort();
    }

    fn get_registration_info(&self) -> Vec<String> {
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

impl DefaultServer {
    fn connect(port: String, data: &HashMap<RequestIdentifier, Value>) -> JoinHandle<()> {
        info!("Updating connection on port {}.", &port);

        let router = DefaultServer::create_router(port.clone(), data);

        tokio::spawn(async move {
            let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
                .await
                .unwrap();

            serve(listener, router).await.unwrap();
        })
    }

    fn create_router(port: String, data: &HashMap<RequestIdentifier, Value>) -> Router {
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
}

#[derive(Default)]
pub struct DefaultServerFactory;

impl ServerFactory<DefaultServer> for DefaultServerFactory {
    fn create(
        app_state: &AppState<DefaultServer, DefaultServerFactory>,
        registration_request: RegistrationRequest,
    ) -> DefaultServer {
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

        let request_identifier = RequestIdentifier::new(path.to_string(), method);

        let mut data = app_state
            .remove_server(&port)
            .map(|server| server.data)
            .unwrap_or(HashMap::new());
        data.insert(request_identifier, response);

        let connection = DefaultServer::connect(port.clone(), &data);

        DefaultServer {
            connection,
            port,
            data,
        }
    }
}
