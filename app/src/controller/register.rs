use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use tracing::{Instrument, info_span};

use crate::{
    business::{
        app_state::AppState,
        server::{
            connection_establisher::ConnectionEstablisher,
            restartable::Restartable,
        },
    },
    model::{
        internal::request_json::RequestJson,
        request::registration_request::RegistrationRequest,
        response::{
            http_response::HttpResponse,
            registration_response::RegistrationResponse,
        },
    },
};

pub async fn register_endpoint_controller<T: ConnectionEstablisher>(
    State(app_state): State<Arc<AppState<T>>>,
    RequestJson(registration_request): RequestJson<RegistrationRequest>,
) -> HttpResponse<RegistrationResponse> {
    let span = info_span!("[Controller: Register Endpoint]");

    async move {
        let port = registration_request.port.to_string();

        let server = app_state.remove_server(&port);

        let registration_to_be_removed = match &server {
            Some(server) => server.get_registration(
                registration_request.path.clone(),
                registration_request.method.clone(),
            ),
            None => None,
        };

        let server = server
            .restart(
                app_state.get_connection_establisher(),
                registration_request.clone(),
            )
            .await;

        match server {
            Ok(server) => {
                app_state.add_server(&port, server);

                let response = RegistrationResponse::new(
                    registration_request,
                    registration_to_be_removed,
                );
                HttpResponse::success(StatusCode::OK, response)
            }
            Err(err) => HttpResponse::failure(StatusCode::BAD_REQUEST, err),
        }
    }
    .instrument(span)
    .await
}
