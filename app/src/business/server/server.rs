use axum::{
    Json, Router,
    routing::{delete, get, patch, post, put},
};
use std::collections::HashMap;

use serde_json::Value;
use tokio::task::JoinHandle;
use tracing::info;

use crate::{
    business::server::{
        RegistrationIdentifier, connection_establisher::ConnectionEstablisher,
        restartable::Restartable,
    },
    logging::http_trace::HttpTracingMiddleware,
    model::{
        error::Error,
        http_method::HttpMethod,
        internal::server_registration::{Registration, ServerRegistration},
        request::registration_request::RegistrationRequest,
    },
};

pub struct Server {
    connection: JoinHandle<()>,
    port: String,
    data: HashMap<RegistrationIdentifier, Value>,
}

impl Server {
    async fn restart<T, F>(
        connection_establisher: &T,
        RegistrationRequest {
            port,
            method,
            path,
            response,
        }: RegistrationRequest,
        data_producer: F,
    ) -> Result<Self, Error>
    where
        T: ConnectionEstablisher,
        F: FnOnce(
            RegistrationIdentifier,
            Value,
        ) -> HashMap<RegistrationIdentifier, Value>,
    {
        info!(%port, %method, %path, "Registering route [{method} (@{port})] {path}.");

        let registration_identifier =
            RegistrationIdentifier::new(path.to_string(), method.clone());

        let data = data_producer(registration_identifier, response);
        let router = Server::create_router(port.clone(), &data);
        let connection =
            connection_establisher.connect(port.clone(), router).await?;

        Ok(Server {
            connection,
            port,
            data,
        })
    }

    fn create_router(
        port: String,
        data: &HashMap<RegistrationIdentifier, Value>,
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

    pub fn stop(&self) {
        self.connection.abort();
    }

    pub fn get_registration(
        &self,
        path: String,
        method: HttpMethod,
    ) -> Option<Registration> {
        let registration_identifier =
            RegistrationIdentifier::new(path.clone(), method.clone());
        let response =
            self.data.get(&registration_identifier).map(|r| r.clone());

        match response {
            Some(response) => Some(Registration::new(method, path, response)),
            None => None,
        }
    }

    pub fn get_registrations(&self) -> ServerRegistration {
        let port = &self.port;

        info!(%port, "Collection information about registrations at server on port {port}.");

        let mut registrations = vec![];

        for (identifier, response) in &self.data {
            registrations.push(Registration::new(
                identifier.method.clone(),
                identifier.path.clone(),
                response.clone(),
            ));
        }

        ServerRegistration::new(port.clone(), registrations)
    }
}

impl<T: ConnectionEstablisher> Restartable<T> for Option<Server> {
    type Instance = Result<Server, Error>;

    async fn restart(
        self,
        connection_establisher: &T,
        registration_request: RegistrationRequest,
    ) -> Self::Instance {
        let port = &registration_request.port;

        if self.is_some() {
            info!(%port, "Restarting the server on port {port}.");
        } else {
            info!(%port, "Starting a server on port {port}.");
        }

        let mut data = match self {
            Some(mut server) => std::mem::take(&mut server.data),
            None => HashMap::new(),
        };

        Server::restart(
            connection_establisher,
            registration_request,
            move |registration_identifier, response| {
                data.insert(registration_identifier, response);
                data
            },
        )
        .await
    }
}
