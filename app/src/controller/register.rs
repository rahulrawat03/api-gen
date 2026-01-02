use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use tracing::{Instrument, info_span};

use crate::{
    business::{
        app_state::AppState,
        server::{
            connection_establisher::ConnectionEstablisher, server::Server,
        },
    },
    model::{
        internal::request_json::RequestJson,
        request::registration_request::RegistrationRequest,
        response::http_response::HttpResponse,
    },
};

pub async fn register_endpoint_controller<T: ConnectionEstablisher>(
    State(app_state): State<Arc<AppState<T>>>,
    RequestJson(registration_request): RequestJson<RegistrationRequest>,
) -> HttpResponse<()> {
    let span = info_span!("[Controller: Register Endpoint]");

    async move {
        let port = registration_request.port.to_string();

        let server = Server::create(&app_state, registration_request).await;

        match server {
            Ok(server) => {
                app_state.add_server(&port, server);
                HttpResponse::success(StatusCode::OK, ())
            }
            Err(err) => HttpResponse::failure(StatusCode::BAD_REQUEST, err),
        }
    }
    .instrument(span)
    .await
}
