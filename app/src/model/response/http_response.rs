use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub struct HttpResponse<T: IntoResponse> {
    status_code: StatusCode,
    data: T,
}

impl<T: IntoResponse> HttpResponse<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self { status_code, data }
    }
}

impl<T: IntoResponse> IntoResponse for HttpResponse<T> {
    fn into_response(self) -> Response {
        (self.status_code, self.data).into_response()
    }
}
