use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use tracing::info_span;

use crate::{
    business::{
        app_state::AppState,
        server::{Server, ServerFactory},
    },
    model::{
        internal::request_json::RequestJson, request::registration_request::RegistrationRequest,
        response::http_response::HttpResponse,
    },
};

pub async fn register_endpoint_controller<S: Server, T: ServerFactory<S>>(
    State(app_state): State<Arc<AppState<S, T>>>,
    RequestJson(registration_request): RequestJson<RegistrationRequest>,
) -> HttpResponse<()> {
    let _entered = info_span!("[Controller: Register Endpoint]").entered();

    let port = registration_request.port.to_string();

    let server = app_state.create_server(registration_request);
    app_state.add_server(&port, server);

    HttpResponse::new(StatusCode::OK, ())
}
