use std::sync::Arc;

use ringbuffer::AllocRingBuffer;
use tokio::sync::RwLock;
use tower_http::add_extension::AddExtensionLayer;
use axum::{Router, routing::get};
use index::index_handler;
use dashboard_info::dashboard_info_handler;
use log_table::log_table_handler;

use crate::LogEntry;
use self::index::static_handler;

mod index;
mod dashboard_info;
mod log_table;

pub fn router_setup(log_array: Arc<RwLock<AllocRingBuffer<LogEntry>>>) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/assets/*file", get(static_handler))
        .route("/api/dashboard_info", get(dashboard_info_handler))
        .route("/api/log_table", get(log_table_handler))
        .layer(AddExtensionLayer::new(log_array))
}