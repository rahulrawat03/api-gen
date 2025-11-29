use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
    serve,
};
use tokio::net::TcpListener;

use crate::{app_state::AppState, controller::register::register_endpoint_controller};

mod app_state;
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
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    serve(listener, app).await.unwrap();
}
