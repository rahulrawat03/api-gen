use std::{collections::HashMap, sync::Arc};

use axum::{Json, extract::State};
use tracing::info_span;

use crate::business::app_state::AppState;

pub async fn list_all_registrations_controller(
    State(app_state): State<Arc<AppState>>,
) -> Json<HashMap<String, Vec<String>>> {
    let _entered = info_span!("[Controller: List All Registrations]").entered();

    let registrations = app_state.get_registration_info();

    Json(registrations)
}
