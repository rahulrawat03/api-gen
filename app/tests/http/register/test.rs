use api_gen::model::http_method::HttpMethod;
use http::StatusCode;
use serde_json::json;

use crate::http::register::{registrar::Registrar, util::app};

#[tokio::test]
async fn should_succeed_for_valid_payload() {
    let (mut router, _) = app();

    router
        .register(
            json!({
                "port": "3000",
                "method": "GET",
                "path": "/hello",
                "response": "Hello World!"
            }),
            |status_code, registration_response| {
                assert_eq!(StatusCode::OK, status_code);
                assert_eq!(
                    json!({
                        "added": {
                            "method": "GET",
                            "path": "/hello",
                            "response": "Hello World!",
                        },
                        "removed": null
                    }),
                    registration_response
                );
            },
        )
        .await;
}

#[tokio::test]
async fn should_accept_http_method_in_case_insensitive_fashion() {
    let (mut router, _) = app();

    router
        .register(
            json!({
                "port": "3000",
                "method": "PosT",
                "path": "/hello",
                "response": "Hello World!",
            }),
            |status_code, registration_response| {
                assert_eq!(StatusCode::OK, status_code);
                assert_eq!(
                    json!({
                        "added": {
                            "method": "POST",
                            "path": "/hello",
                            "response": "Hello World!",
                        },
                        "removed": null
                    }),
                    registration_response
                );
            },
        )
        .await;
}

#[tokio::test]
async fn should_fail_for_invalid_method() {
    let (mut router, _) = app();

    router
        .register(
            json!({
                "port": "3000",
                "method": "INVALID_METHOD",
                "path": "/hello",
                "response": "Hello World!",
            }),
            |status_code, _| {
                assert_eq!(StatusCode::BAD_REQUEST, status_code);
            },
        )
        .await;
}

#[tokio::test]
async fn should_fail_for_missing_attributes() {
    let (mut router, _) = app();

    router
        .register(
            json!({
                "method": "GET",
                "path": "/hello",
                "response": "Hello World!",
            }),
            |status_code, _| {
                assert_eq!(StatusCode::BAD_REQUEST, status_code);
            },
        )
        .await;

    router
        .register(
            json!({
                "port": "3000",
                "path": "/hello",
                "response": "Hello World!",
            }),
            |status_code, _| {
                assert_eq!(StatusCode::BAD_REQUEST, status_code);
            },
        )
        .await;

    router
        .register(
            json!({
                "port": "3000",
                "method": "GET",
                "response": "Hello World!",
            }),
            |status_code, _| {
                assert_eq!(StatusCode::BAD_REQUEST, status_code);
            },
        )
        .await;

    router
        .register(
            json!({
                "port": "3000",
                "path": "/hello",
                "method": "GET",
            }),
            |status_code, _| {
                assert_eq!(StatusCode::BAD_REQUEST, status_code);
            },
        )
        .await;
}

#[tokio::test]
async fn should_respond_at_registered_endpoint() {
    let (mut router, registration_verifier_builder) = app();

    router
        .register(
            json!({
                "port": "3000",
                "method": "GET",
                "path": "/hello",
                "response": "Hello World!",
            }),
            |status_code, registration_response| {
                assert_eq!(StatusCode::OK, status_code);
                assert_eq!(
                    json!({
                        "added": {
                            "method": "GET",
                            "path": "/hello",
                            "response": "Hello World!",
                        },
                        "removed": null
                    }),
                    registration_response
                );
            },
        )
        .await;

    let registration_verifier = registration_verifier_builder
        .port("3000")
        .method(HttpMethod::Get)
        .path("/hello")
        .build();

    registration_verifier
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(json!("Hello World!"), response_body);
        })
        .await;
}

#[tokio::test]
async fn should_override_registered_response_with_new_one() {
    let (mut router, registration_verifier_builder) = app();

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
                    "method": "GET",
                    "path": "/hello",
                    "response": "Hello World!!!"
                },
            ]),
            |idx, status_code, registration_response| {
                assert_eq!(StatusCode::OK, status_code);

                if idx == 0 {
                    assert_eq!(
                        json!({
                            "added": {
                                "method": "GET",
                                "path": "/hello",
                                "response": "Hello World!",
                            },
                            "removed": null,
                        }),
                        registration_response
                    );
                } else if idx == 1 {
                    assert_eq!(
                        json!({
                            "added": {
                                "method": "GET",
                                "path": "/hello",
                                "response": "Hello World!!!",
                            },
                            "removed": {
                                "method": "GET",
                                "path": "/hello",
                                "response": "Hello World!"
                            },
                        }),
                        registration_response
                    );
                }
            },
        )
        .await;

    let registration_verifier = registration_verifier_builder
        .port("3000")
        .method(HttpMethod::Get)
        .path("/hello")
        .build();

    registration_verifier
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_ne!(json!("Hello World!"), response_body);
            assert_eq!(json!("Hello World!!!"), response_body);
        })
        .await;
}

#[tokio::test]
async fn should_handle_different_ports_independently() {
    let (mut router, registration_verifier_builder) = app();

    router
        .register_many(
            json!([
                {
                    "port": "3000",
                    "method": "GET",
                    "path": "/hello",
                    "response": "[3000: GET](/hello) Hello World!",
                },
                {
                    "port": "3001",
                    "method": "GET",
                    "path": "/hello",
                    "response": "[3001: GET](/hello) Hello World!"
                },
                {
                    "port": "3002",
                    "method": "GET",
                    "path": "/hello",
                    "response": "[3002: GET](/hello) Hello World!"
                },
            ]),
            |_, status_code, _| {
                assert_eq!(StatusCode::OK, status_code);
            },
        )
        .await;

    let registration_verifier_builder = registration_verifier_builder
        .method(HttpMethod::Get)
        .path("/hello");

    let registration_verifier_builder =
        registration_verifier_builder.port("3000");
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3000: GET](/hello) Hello World!"),
                response_body
            );
        })
        .await;

    let registration_verifier_builder =
        registration_verifier_builder.port("3001");
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3001: GET](/hello) Hello World!"),
                response_body
            );
        })
        .await;

    let registration_verifier_builder =
        registration_verifier_builder.port("3002");
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3002: GET](/hello) Hello World!"),
                response_body
            );
        })
        .await;
}

#[tokio::test]
async fn should_handle_different_methods_independently() {
    let (mut router, registration_verifier_builder) = app();

    router
        .register_many(
            json!([
                {
                    "port": "3000",
                    "method": "GET",
                    "path": "/hello",
                    "response": "[3000: GET](/hello) Hello World!",
                },
                {
                    "port": "3000",
                    "method": "POST",
                    "path": "/hello",
                    "response": "[3000: POST](/hello) Hello World!"
                },
                {
                    "port": "3000",
                    "method": "PUT",
                    "path": "/hello",
                    "response": "[3000: PUT](/hello) Hello World!"
                },
                {
                    "port": "3000",
                    "method": "PATCH",
                    "path": "/hello",
                    "response": "[3000: PATCH](/hello) Hello World!"
                },
                {
                    "port": "3000",
                    "method": "DELETE",
                    "path": "/hello",
                    "response": "[3000: DELETE](/hello) Hello World!"
                },
            ]),
            |_, status_code, _| {
                assert_eq!(StatusCode::OK, status_code);
            },
        )
        .await;

    let registration_verifier_builder =
        registration_verifier_builder.port("3000").path("/hello");

    let registration_verifier_builder =
        registration_verifier_builder.method(HttpMethod::Get);
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3000: GET](/hello) Hello World!"),
                response_body
            );
        })
        .await;

    let registration_verifier_builder =
        registration_verifier_builder.method(HttpMethod::Post);
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3000: POST](/hello) Hello World!"),
                response_body
            );
        })
        .await;

    let registration_verifier_builder =
        registration_verifier_builder.method(HttpMethod::Put);
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3000: PUT](/hello) Hello World!"),
                response_body
            );
        })
        .await;

    let registration_verifier_builder =
        registration_verifier_builder.method(HttpMethod::Patch);
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3000: PATCH](/hello) Hello World!"),
                response_body
            );
        })
        .await;

    let registration_verifier_builder =
        registration_verifier_builder.method(HttpMethod::Delete);
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3000: DELETE](/hello) Hello World!"),
                response_body
            );
        })
        .await;
}

#[tokio::test]
async fn should_handle_different_paths_differently() {
    let (mut router, registration_verifier_builder) = app();

    router
        .register_many(
            json!([
                {
                    "port": "3000",
                    "method": "GET",
                    "path": "/hello1",
                    "response": "[3000: GET](/hello1) Hello World!",
                },
                {
                    "port": "3000",
                    "method": "GET",
                    "path": "/hello2",
                    "response": "[3000: GET](/hello2) Hello World!"
                },
                {
                    "port": "3000",
                    "method": "GET",
                    "path": "/hello3",
                    "response": "[3000: GET](/hello3) Hello World!"
                },
            ]),
            |_, status_code, _| {
                assert_eq!(StatusCode::OK, status_code);
            },
        )
        .await;

    let registration_verifier_builder = registration_verifier_builder
        .port("3000")
        .method(HttpMethod::Get);

    let registration_verifier_builder =
        registration_verifier_builder.path("/hello1");
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3000: GET](/hello1) Hello World!"),
                response_body
            );
        })
        .await;

    let registration_verifier_builder =
        registration_verifier_builder.path("/hello2");
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3000: GET](/hello2) Hello World!"),
                response_body
            );
        })
        .await;

    let registration_verifier_builder =
        registration_verifier_builder.path("/hello3");
    registration_verifier_builder
        .build()
        .request(|status_code, response_body| {
            assert_eq!(StatusCode::OK, status_code);
            assert_eq!(
                json!("[3000: GET](/hello3) Hello World!"),
                response_body
            );
        })
        .await;
}
