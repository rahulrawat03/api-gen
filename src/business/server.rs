use std::collections::HashMap;

use axum::{
    Json, Router,
    routing::{get, post},
    serve,
};
use serde_json::Value;
use tokio::{net::TcpListener, task::JoinHandle};

use crate::{
    app_state::AppState,
    model::{http_method::HttpMethod, request::registration_request::RegistrationRequest},
};

#[derive(PartialEq, Eq, Hash)]
pub struct RequestIdentifier {
    pub path: String,
    pub method: HttpMethod,
}

impl RequestIdentifier {
    pub fn new(path: String, method: HttpMethod) -> Self {
        Self { path, method }
    }
}

pub struct Server {
    connection: JoinHandle<()>,
    data: HashMap<RequestIdentifier, Value>,
}

impl Server {
    pub fn register_route(app_state: &AppState, registration_request: RegistrationRequest) -> Self {
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

        let connection = Server::connect(&port, &data);

        Self { connection, data }
    }

    fn connect(port: &str, data: &HashMap<RequestIdentifier, Value>) -> JoinHandle<()> {
        let router = Server::create_router(data);
        let port = port.to_string();

        tokio::spawn(async move {
            let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
                .await
                .unwrap();

            serve(listener, router).await.unwrap();
        })
    }

    fn create_router(data: &HashMap<RequestIdentifier, Value>) -> Router {
        let mut router = Router::new();

        for (request_identifier, response) in data {
            let response = Json(response.clone());

            let method_router = match &request_identifier.method {
                HttpMethod::Get => get(async || response),
                HttpMethod::Post => post(async || response),
                HttpMethod::Put => post(async || response),
                HttpMethod::Patch => post(async || response),
                HttpMethod::Delete => post(async || response),
            };

            router = router.route(&request_identifier.path, method_router)
        }

        router
    }

    pub fn stop(&self) {
        self.connection.abort();
    }
}
