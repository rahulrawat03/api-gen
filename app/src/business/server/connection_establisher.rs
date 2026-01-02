use std::sync::Arc;

use axum::{Router, serve};
use tokio::{net::TcpListener, task::JoinHandle};
use tracing::info;

use crate::{model::error::Error, util::notifier::Notifier};

pub trait ConnectionEstablisher: Send + Sync {
    fn connect(
        &self,
        port: String,
        router: Router,
    ) -> impl Future<Output = Result<JoinHandle<()>, Error>> + Send + Sync;
}

#[derive(Default)]
pub struct TcpConnectionEstablisher;

impl ConnectionEstablisher for TcpConnectionEstablisher {
    async fn connect(
        &self,
        port: String,
        router: Router,
    ) -> Result<JoinHandle<()>, Error> {
        info!(port = port, "Establishing connectio non port {}.", port);

        let notifier = Arc::new(Notifier::new());

        let notifier_clone = notifier.clone();
        let join_handle = tokio::spawn(async move {
            match TcpListener::bind(format!("0.0.0.0:{}", port)).await {
                Ok(listener) => {
                    let _ = notifier_clone.notify(Ok(()));
                    serve(listener, router).await.unwrap();
                }
                Err(err) => {
                    let _ = notifier_clone.notify(Err(err));
                }
            };
        });

        match notifier.await_notification().await {
            Ok(Ok(_)) => Ok(join_handle),
            Ok(Err(err)) => Err(Error::Connection(format!(
                "Failed to establish connection, {}",
                err
            ))),
            Err(_notification_error) => {
                Err(Error::Connection("Something went wrong!".to_string()))
            }
        }
    }
}
