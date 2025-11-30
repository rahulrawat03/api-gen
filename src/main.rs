use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
    serve,
};
use tokio::net::TcpListener;

use crate::{
    business::app_state::AppState,
    controller::{
        register::register_endpoint_controller, registrations::list_all_registrations_controller,
    },
    logging::{http_trace::HttpTracingMiddleware, setup::setup_logging},
};

mod business;
mod controller;
mod logging;
mod model;
mod util;

#[tokio::main]
async fn main() {
    setup_logging();

    let app_state = Arc::new(AppState::new());
    let port = "8080";

    let app = Router::new()
        .route("/health", get(|| async { "Up and running..." }))
        .route("/register", post(register_endpoint_controller))
        .route("/info", get(list_all_registrations_controller))
        .with_state(app_state)
        .with_http_tracing(port.to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    serve(listener, app).await.unwrap();
}
