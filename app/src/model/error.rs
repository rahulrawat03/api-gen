use crate::model::response::http_response::HttpResponse;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::{Value, json};

pub enum Error {
    JsonParse(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let http_response = match self {
            Self::JsonParse(error_message) => {
                let data = Error::json("MalformedJson", &error_message);
                HttpResponse::new(StatusCode::OK, data)
            }
        };

        http_response.into_response()
    }
}

impl Error {
    fn json(failure_type: &str, failure_message: &str) -> Value {
        json!({
            "status": "FAILED",
            "failureType": failure_type,
            "failureMessage": failure_message,
        })
    }
}
