use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use http::StatusCode;
use tracing::error;

use crate::model::error::Error;

pub struct RequestJson<T>(pub T);

impl<S, T> FromRequest<S> for RequestJson<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Error);

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let method = request.method().to_string();
        let uri = request.uri().to_string();

        match Json::<T>::from_request(request, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let error_message = rejection.body_text();
                error!(
                    "Unexpected JSON received in body for [{}]({}), {}",
                    method.to_string(),
                    uri.to_string(),
                    error_message.clone(),
                );

                let error = Error::JsonParse(error_message);
                Err((StatusCode::BAD_REQUEST, error))
            }
        }
    }
}
