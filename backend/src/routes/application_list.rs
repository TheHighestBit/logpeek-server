use axum::extract::State;
use axum::Json;
use log::trace;
use crate::SharedState;

pub async fn application_list_handler(State(shared_state): State<SharedState>) -> Json<Vec<String>> {
    trace!("Request received");

    let i_to_app = shared_state.i_to_app.lock().await;
    let app_list: Vec<String> = i_to_app.values().cloned().collect();

    Json(app_list)
}