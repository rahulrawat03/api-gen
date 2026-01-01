use axum::{Router, serve};
use tokio::{net::TcpListener, task::JoinHandle};
use tracing::info;

pub trait ConnectionEstablisher {
    fn connect(&self, port: String, router: Router) -> JoinHandle<()>;
}

#[derive(Default)]
pub struct TcpConnectionEstablisher;

impl ConnectionEstablisher for TcpConnectionEstablisher {
    fn connect(&self, port: String, router: Router) -> JoinHandle<()> {
        info!(port = port, "Establishing connectio non port {}.", port);

        tokio::spawn(async move {
            let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
                .await
                .unwrap();

            serve(listener, router).await.unwrap();
        })
    }
}
