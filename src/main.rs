use api_gen::app;
use axum::serve;
use tokio::net::TcpListener;

use api_gen::logging::setup::setup_logging;

#[tokio::main]
async fn main() {
    setup_logging();

    let port = "8080";
    let app = app(port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    serve(listener, app).await.unwrap();
}
