use api_gen::model::http_method::HttpMethod;
use axum::{Router, body::Body, extract::Request};
use http::StatusCode;
use serde_json::{Value, json};
use tower::ServiceExt;

const REGISTER_ENDPOINT: &'static str = "/register";

#[tokio::test]
async fn should_succeed_for_valid_payload() {
    let app = app();

    let request = build_request(
        HttpMethod::Post,
        json!({
            "port": "3000",
            "method": "GET",
            "path": "/test",
            "response": "Hello World!"
        }),
    );
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(StatusCode::OK, response.status());
}

#[tokio::test]
async fn should_accept_http_method_in_case_insensitive_fashion() {
    let app = app();

    let request = build_request(
        HttpMethod::Post,
        json!({
            "port": "3000",
            "method": "PosT",
            "path": "/test",
            "response": "Hello World!",
        }),
    );
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(StatusCode::OK, response.status());
}

#[tokio::test]
async fn should_fail_for_payload_with_missing_attributes() {
    let app = app();

    let request = build_request(
        HttpMethod::Post,
        json!({
            "port": "3000",
            "method": "INVALID_METHOD",
        }),
    );
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

fn app() -> Router {
    api_gen::app("8080")
}

fn build_request(method: HttpMethod, body: Value) -> Request {
    let body = body.to_string();
    let body = Body::from(body);

    Request::builder()
        .uri(REGISTER_ENDPOINT)
        .method(method.to_string().as_str())
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}
