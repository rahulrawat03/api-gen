use axum::Router;

use crate::http::{
    register::registration_verifier::{
        RegistrationVerifier, RegistrationVerifierBuilder,
    },
    util,
};

pub(super) fn app() -> (Router, RegistrationVerifierBuilder) {
    let (router, connection_establisher) = util::app();
    let registration_verifier_builder =
        RegistrationVerifier::builder(connection_establisher);

    (router, registration_verifier_builder)
}
