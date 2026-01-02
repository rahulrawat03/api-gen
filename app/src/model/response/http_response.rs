use axum::{
    Json,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use serde::Serialize;

use crate::model::error::Error;

pub enum HttpResponse<T: Serialize> {
    Success(StatusCode, Json<T>),
    Failure(StatusCode, Error),
}

impl<T: Serialize> HttpResponse<T> {
    pub fn success(status_code: StatusCode, data: T) -> Self {
        Self::Success(status_code, Json(data))
    }

    pub fn failure(status_code: StatusCode, error: Error) -> Self {
        Self::Failure(status_code, error)
    }
}

impl<T: Serialize> IntoResponse for HttpResponse<T> {
    fn into_response(self) -> Response {
        match self {
            Self::Success(status_code, json) => {
                (status_code, json).into_response()
            }
            Self::Failure(status_code, error) => {
                (status_code, error).into_response()
            }
        }
    }
}
