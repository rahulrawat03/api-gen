use api_gen::model::http_method::HttpMethod;
use http::StatusCode;
use serde_json::Value;

use crate::{
    http::request_sender::RequestSender,
    test_double::fake_connection_establisher::FakeConnectionEstablisher,
};

pub(super) struct RegistrationVerifier {
    connection_establisher: FakeConnectionEstablisher,
    port: String,
    method: HttpMethod,
    path: String,
    body: Option<Value>,
}

impl RegistrationVerifier {
    pub(super) async fn request<F>(&self, assertion: F)
    where
        F: Fn(StatusCode, Value),
    {
        let mut router = self.connection_establisher.get_router(&self.port);

        let (status_code, response_body) = router
            .send(
                self.path.to_string(),
                self.method.clone(),
                self.body.clone(),
            )
            .await;

        assertion(status_code, response_body);
    }

    pub(super) fn builder(
        connection_establisher: FakeConnectionEstablisher,
    ) -> RegistrationVerifierBuilder {
        RegistrationVerifierBuilder::new(connection_establisher)
    }
}

pub(super) struct RegistrationVerifierBuilder {
    connection_establisher: FakeConnectionEstablisher,
    port: Option<String>,
    method: Option<HttpMethod>,
    path: Option<String>,
    body: Option<Value>,
}

impl RegistrationVerifierBuilder {
    fn new(connection_establisher: FakeConnectionEstablisher) -> Self {
        Self {
            connection_establisher,
            port: None,
            method: None,
            path: None,
            body: None,
        }
    }

    pub(super) fn port(mut self, port: &str) -> Self {
        self.port = Some(port.to_string());
        self
    }

    pub(super) fn method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);
        self
    }

    pub(super) fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    pub(super) fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    pub(super) fn build(&self) -> RegistrationVerifier {
        self.validate();

        RegistrationVerifier {
            connection_establisher: self.connection_establisher.clone(),
            port: self.port.clone().unwrap(),
            method: self.method.clone().unwrap(),
            path: self.path.clone().unwrap(),
            body: self.body.clone(),
        }
    }

    fn validate(&self) {
        if self.port.is_none() {
            panic!("`port` cannot be null!");
        }

        if self.method.is_none() {
            panic!("`method` cannot be null!");
        }

        if self.path.is_none() {
            panic!("`path` cannot be null!");
        }
    }
}
