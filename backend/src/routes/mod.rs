use std::sync::Arc;

use tokio::sync::RwLock;
use tower_http::add_extension::AddExtensionLayer;
use axum::{Router, routing::get};
use index_handler::index_handler;
use dashboard_info::dashboard_info_handler;

use crate::LogEntry;

use self::index_handler::static_handler;

mod index_handler;
mod dashboard_info;

pub fn router_setup(log_array: Arc<RwLock<Vec<LogEntry>>>) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/assets/*file", get(static_handler))
        .route("/api/dashboard_info", get(dashboard_info_handler))
        .layer(AddExtensionLayer::new(log_array))
}