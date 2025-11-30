use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    business::{app_state::AppState, server::Server},
    model::request::registration_request::RegistrationRequest,
};

pub async fn register_endpoint_controller(
    State(app_state): State<Arc<AppState>>,
    Json(registration_request): Json<RegistrationRequest>,
) -> StatusCode {
    let port = registration_request.port.to_string();

    let server = Server::register_route(&app_state, registration_request);
    app_state.add_server(&port, server);

    StatusCode::OK
}
