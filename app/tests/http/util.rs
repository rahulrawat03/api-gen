use std::sync::Arc;

use api_gen::business::app_state::AppState;
use axum::Router;

use crate::test_double::fake_connection_establisher::FakeConnectionEstablisher;

const DEFAULT_APPLICATION_PORT: &'static str = "8080";

pub(super) fn app() -> (Router, FakeConnectionEstablisher) {
    let connection_establisher = FakeConnectionEstablisher::new();
    let app_state = Arc::new(AppState::new(connection_establisher.clone()));

    (
        api_gen::app(DEFAULT_APPLICATION_PORT, app_state.clone()),
        connection_establisher,
    )
}
