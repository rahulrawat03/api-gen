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
};

mod business;
mod controller;
mod model;
mod util;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/health", get(|| async { "Up and running..." }))
        .route("/register", post(register_endpoint_controller))
        .route("/info", get(list_all_registrations_controller))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    serve(listener, app).await.unwrap();
}
