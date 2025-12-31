use api_gen::model::http_method::HttpMethod;
use axum::{Router, body::Body, extract::Request, response::Response};
use http::StatusCode;
use http_body_util::BodyExt;
use serde_json::Value;
use tower::{Service, ServiceExt};

pub(super) trait RequestSender {
    fn send(
        &mut self,
        uri: String,
        method: HttpMethod,
        body: Option<Value>,
    ) -> impl Future<Output = (StatusCode, Value)>;

    fn build_request(
        &self,
        uri: String,
        method: HttpMethod,
        body: Option<Value>,
    ) -> Request;
}

impl RequestSender for Router {
    async fn send(
        &mut self,
        uri: String,
        method: HttpMethod,
        body: Option<Value>,
    ) -> (StatusCode, Value) {
        let request = self.build_request(uri, method, body);
        let service = <Router as ServiceExt<Request>>::ready(self)
            .await
            .expect("Failed to make service ready!");
        let response = service
            .call(request)
            .await
            .expect("Couldn't make the request!");

        let status_code = response.status();
        let response_body = response.json().await;

        (status_code, response_body)
    }

    fn build_request(
        &self,
        uri: String,
        method: HttpMethod,
        body: Option<Value>,
    ) -> Request {
        let body = match body {
            Some(body) => Body::from(body.to_string()),
            None => Body::empty(),
        };

        Request::builder()
            .uri(uri)
            .method(method.to_string().as_str())
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}

trait ResponseExtractor {
    fn json(self) -> impl Future<Output = Value>;
}

impl ResponseExtractor for Response<Body> {
    async fn json(self) -> Value {
        let body = self.into_body();
        let collection = body
            .collect()
            .await
            .expect("Failed to collect the body from response!");
        let bytes = collection.to_bytes();

        if bytes.len() == 0 {
            Value::Null
        } else {
            serde_json::from_slice::<Value>(&bytes)
                .expect("Failed to deserialize body JSON!")
        }
    }
}
