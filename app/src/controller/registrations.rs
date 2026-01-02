use std::sync::Arc;

use axum::extract::State;
use http::StatusCode;
use tracing::info_span;

use crate::{
    business::{
        app_state::AppState,
        server::connection_establisher::ConnectionEstablisher,
    },
    model::{
        internal::server_registration::ServerRegistration,
        response::http_response::HttpResponse,
    },
};

pub async fn list_all_registrations_controller<T: ConnectionEstablisher>(
    State(app_state): State<Arc<AppState<T>>>,
) -> HttpResponse<Vec<ServerRegistration>> {
    let _entered = info_span!("[Controller: List All Registrations]").entered();

    let registrations = app_state.get_registrations();

    HttpResponse::success(StatusCode::OK, registrations)
}
