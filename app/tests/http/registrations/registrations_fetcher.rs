use api_gen::model::{
    http_method::HttpMethod, response::server_registration::ServerRegistration,
};
use axum::Router;
use http::StatusCode;

use crate::http::request_sender::RequestSender;

const LIST_REGISTRATIONS_ENDPOINT: &'static str = "/info";

pub(super) trait RegistrationsFetcher {
    fn fetch_registrations<F>(
        &mut self,
        assertion: F,
    ) -> impl Future<Output = ()>
    where
        F: Fn(StatusCode, Vec<ServerRegistration>);
}

impl RegistrationsFetcher for Router {
    async fn fetch_registrations<F>(&mut self, assertion: F)
    where
        F: Fn(StatusCode, Vec<ServerRegistration>),
    {
        let (status_code, registrations) = self
            .send(
                LIST_REGISTRATIONS_ENDPOINT.to_string(),
                HttpMethod::Get,
                None,
            )
            .await;

        let registrations =
            serde_json::from_value::<Vec<ServerRegistration>>(registrations)
                .expect("Failed to deserialize response JSON!");

        assertion(status_code, registrations);
    }
}
