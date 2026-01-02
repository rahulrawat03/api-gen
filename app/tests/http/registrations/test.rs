use api_gen::model::http_method::HttpMethod;
use http::StatusCode;
use serde_json::json;

use crate::http::{
    register::registrar::Registrar,
    registrations::{
        registrations_fetcher::RegistrationsFetcher,
        server_registration_extensions::ServerRegistrationExtensions,
    },
    util::app,
};

#[tokio::test]
async fn should_list_all_registrations() {
    let (mut router, _) = app();

    router
        .register_many(
            json!([
                {
                    "port": "3000",
                    "method": "GET",
                    "path": "/hello",
                    "response": "Hello World!",
                },
                {
                    "port": "3000",
                    "method": "POST",
                    "path": "/hello",
                    "response": "Hello World!!!",
                },
                {
                    "port": "3001",
                    "method": "GET",
                    "path": "/hello",
                    "response": "Hello World!",
                },
            ]),
            |_, _, _| {},
        )
        .await;

    router
        .fetch_registrations(|status_code, registrations| {
            assert_eq!(StatusCode::OK, status_code);

            assert_eq!(2, registrations.len());

            assert!(matches!(
                registrations.find(
                    "3000",
                    HttpMethod::Get,
                    "/hello",
                    json!("Hello World!"),
                ),
                Some(_),
            ));
            assert!(matches!(
                registrations.find(
                    "3000",
                    HttpMethod::Post,
                    "/hello",
                    json!("Hello World!!!"),
                ),
                Some(_),
            ));
            assert!(matches!(
                registrations.find(
                    "3001",
                    HttpMethod::Get,
                    "/hello",
                    json!("Hello World!"),
                ),
                Some(_),
            ));
        })
        .await;
}

#[tokio::test]
async fn should_not_list_overriden_registrations() {
    let (mut router, _) = app();

    router
        .register_many(
            json!([
                {
                    "port": "3000",
                    "method": "GET",
                    "path": "/hello",
                    "response": "Hello World!",
                },
                {
                    "port": "3000",
                    "method": "POST",
                    "path": "/hello",
                    "response": "Hello World!!!",
                },
                {
                    "port": "3001",
                    "method": "GET",
                    "path": "/hello",
                    "response": "Hello World!",
                },
            ]),
            |_, _, _| {},
        )
        .await;

    router
        .fetch_registrations(|status_code, registrations| {
            assert_eq!(StatusCode::OK, status_code);

            assert_eq!(2, registrations.len());

            assert!(matches!(
                registrations.find(
                    "3000",
                    HttpMethod::Get,
                    "/hello",
                    json!("Hello World!"),
                ),
                Some(_),
            ));
            assert!(matches!(
                registrations.find(
                    "3000",
                    HttpMethod::Post,
                    "/hello",
                    json!("Hello World!!!"),
                ),
                Some(_),
            ));
            assert!(matches!(
                registrations.find(
                    "3001",
                    HttpMethod::Get,
                    "/hello",
                    json!("Hello World!"),
                ),
                Some(_),
            ));
        })
        .await;

    router
        .register_many(
            json!([
                {
                    "port": "3000",
                    "method": "GET",
                    "path": "/hello",
                    "response": "Hello World! (UPDATED)",
                },
                {
                    "port": "3001",
                    "method": "GET",
                    "path": "/hello",
                    "response": "Hello World! (UPDATED)",
                },
                {
                    "port": "3000",
                    "method": "PATCH",
                    "path": "/hello",
                    "response": "Hello World!!!",
                },
            ]),
            |_, _, _| {},
        )
        .await;

    router
        .fetch_registrations(|status_code, registrations| {
            assert_eq!(StatusCode::OK, status_code);

            assert_eq!(2, registrations.len());

            assert!(matches!(
                registrations.find(
                    "3000",
                    HttpMethod::Get,
                    "/hello",
                    json!("Hello World! (UPDATED)"),
                ),
                Some(_),
            ));
            assert!(matches!(
                registrations.find(
                    "3000",
                    HttpMethod::Post,
                    "/hello",
                    json!("Hello World!!!"),
                ),
                Some(_),
            ));
            assert!(matches!(
                registrations.find(
                    "3000",
                    HttpMethod::Patch,
                    "/hello",
                    json!("Hello World!!!"),
                ),
                Some(_),
            ));
            assert!(matches!(
                registrations.find(
                    "3001",
                    HttpMethod::Get,
                    "/hello",
                    json!("Hello World! (UPDATED)"),
                ),
                Some(_),
            ));
        })
        .await;
}
