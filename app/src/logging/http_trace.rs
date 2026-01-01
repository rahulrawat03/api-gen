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
                let method = request.method();
                let path = request.uri();

                info!(%port, %method, %path, "[Request]: [{method} (@{port})]: {path}.");
            })
            .on_response(
                |response: &Response<Body>, latency: Duration, _: &Span| {
                    let status = response.status();
                    let latency = latency.as_millis();

                    info!(%status, %latency, "[Response]: {status} ({latency}ms).");
                },
            );

        self.layer(trace_layer)
    }
}
