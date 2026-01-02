use api_gen::model::http_method::HttpMethod;
use axum::Router;
use http::StatusCode;
use serde_json::Value;

use crate::http::request_sender::RequestSender;

const REGISTER_ENDPOINT: &'static str = "/register";

pub(crate) trait Registrar {
    fn register<F>(
        &mut self,
        registration_request: Value,
        assertion: F,
    ) -> impl Future<Output = ()>
    where
        F: Fn(StatusCode, Value) -> ();

    fn register_many<F>(
        &mut self,
        registration_request: Value,
        assertion: F,
    ) -> impl Future<Output = ()>
    where
        F: Fn(usize, StatusCode, Value) -> ();
}

impl Registrar for Router {
    async fn register<F>(&mut self, registration_request: Value, assertion: F)
    where
        F: Fn(StatusCode, Value) -> (),
    {
        let (status_code, response_body) = self
            .send(
                REGISTER_ENDPOINT.to_string(),
                HttpMethod::Post,
                Some(registration_request),
            )
            .await;

        assertion(status_code, response_body);
    }

    async fn register_many<F>(
        &mut self,
        registration_request: Value,
        assertion: F,
    ) where
        F: Fn(usize, StatusCode, Value) -> (),
    {
        if let Value::Array(requests) = registration_request {
            for (idx, request) in requests.into_iter().enumerate() {
                self.register(request, |status_code, response_body| {
                    assertion(idx, status_code, response_body)
                })
                .await;
            }
        } else {
            panic!("`register_many` only accepts a list of requests!");
        }
    }
}
