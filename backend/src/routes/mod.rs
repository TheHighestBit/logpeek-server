




use axum::{Router, routing::get};
use index::index_handler;
use dashboard_info::dashboard_info_handler;
use log_table::log_table_handler;
use self::index::static_handler;
use sysinfo::sysinfo_handler;

use crate::{SharedState};

mod index;
mod dashboard_info;
mod log_table;
mod sysinfo;

pub fn router_setup(shared_state: SharedState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/assets/*file", get(static_handler))
        .route("/api/dashboard_info", get(dashboard_info_handler))
        .route("/api/log_table", get(log_table_handler))
        .route("/api/sysinfo", get(sysinfo_handler))
        .with_state(shared_state)
}