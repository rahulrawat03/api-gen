use std::time::Duration;

use axum::{Router, body::Body};
use http::{Request, Response};
use tower_http::trace::TraceLayer;
use tracing::{Span, info, info_span};

pub trait HttpTracingMiddleware {
    fn with_http_tracing(self, port: String) -> Router<()>;
}

impl HttpTracingMiddleware for Router<()> {
    fn with_http_tracing(self, port: String) -> Router<()> {
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(|_: &Request<Body>| info_span!("[HTTP]"))
            .on_request(move |request: &Request<Body>, _: &Span| {
                info!(
                    "[Request]: [{} (@{})]: {}.",
                    request.method(),
                    port,
                    request.uri()
                )
            })
            .on_response(
                |response: &Response<Body>, latency: Duration, _: &Span| {
                    info!(
                        "[Response]: {} ({}ms).",
                        response.status(),
                        latency.as_millis(),
                    )
                },
            );

        self.layer(trace_layer)
    }
}
