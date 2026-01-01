use axum::{
    Json,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use serde::Serialize;

pub struct HttpResponse<T: Serialize> {
    status_code: StatusCode,
    data: Json<T>,
}

impl<T: Serialize> HttpResponse<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code,
            data: Json(data),
        }
    }
}

impl<T: Serialize> IntoResponse for HttpResponse<T> {
    fn into_response(self) -> Response {
        (self.status_code, self.data).into_response()
    }
}
