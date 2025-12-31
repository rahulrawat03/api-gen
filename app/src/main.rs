use std::sync::Arc;

use api_gen::{
    app,
    business::{
        app_state::AppState,
        server::connection_establisher::TcpConnectionEstablisher,
    },
};
use axum::serve;
use tokio::net::TcpListener;

use api_gen::logging::setup::setup_logging;

#[tokio::main]
async fn main() {
    setup_logging();

    let port = "8080";
    let connection_establisher = TcpConnectionEstablisher::default();
    let app_state = Arc::new(AppState::new(connection_establisher));

    let app = app(port, app_state);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    serve(listener, app).await.unwrap();
}
