use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};

pub enum Error {
    JsonParse(String),
    Connection(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let http_response = match self {
            Self::JsonParse(error_message) => {
                Json(Error::json("MalformedJson", &error_message))
                    .into_response()
            }
            Self::Connection(error_message) => {
                Json(Error::json("Connection", &error_message)).into_response()
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
