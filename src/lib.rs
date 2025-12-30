use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    business::{app_state::AppState, server::default_server::DefaultServerFactory},
    controller::{
        register::register_endpoint_controller, registrations::list_all_registrations_controller,
    },
    logging::http_trace::HttpTracingMiddleware,
};

pub mod business;
pub mod controller;
pub mod logging;
pub mod model;
pub mod util;

pub fn app(port: &str) -> Router {
    let default_server_factory = DefaultServerFactory::default();
    let app_state = Arc::new(AppState::new(default_server_factory));

    Router::new()
        .route("/health", get(|| async { "Up and running..." }))
        .route("/register", post(register_endpoint_controller))
        .route("/info", get(list_all_registrations_controller))
        .with_state(app_state)
        .with_http_tracing(port.to_string())
}
