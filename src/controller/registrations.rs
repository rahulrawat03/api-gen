use std::{collections::HashMap, sync::Arc};

use axum::{Json, extract::State};
use tracing::info_span;

use crate::business::{
    app_state::AppState,
    server::{Server, ServerFactory},
};

pub async fn list_all_registrations_controller<S: Server, T: ServerFactory<S>>(
    State(app_state): State<Arc<AppState<S, T>>>,
) -> Json<HashMap<String, Vec<String>>> {
    let _entered = info_span!("[Controller: List All Registrations]").entered();

    let registrations = app_state.get_registration_info();

    Json(registrations)
}
