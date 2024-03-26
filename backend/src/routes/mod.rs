use axum::{Router, routing::get};
use axum::middleware::{from_fn_with_state};
use log::info;
use index::index_handler;
use dashboard_info::dashboard_info_handler;
use log_table::log_table_handler;
use self::index::static_handler;
use sysinfo::sysinfo_handler;

use crate::{middleware, SETTINGS, SharedState};
use crate::routes::application_list::application_list_handler;
use crate::routes::authenticate::authenticate_handler;

mod index;
mod dashboard_info;
mod log_table;
mod sysinfo;
mod authenticate;
mod application_list;

pub async fn router_setup(shared_state: SharedState<>) -> Router {
    let mut router = Router::new()
        .route("/api/dashboard_info", get(dashboard_info_handler))
        .route("/api/log_table", get(log_table_handler))
        .route("/api/authenticate", get(authenticate_handler))
        .layer(from_fn_with_state(shared_state.clone(), middleware::buffer_refresh_middleware))
        .route("/api/sysinfo", get(sysinfo_handler))
        .route("/api/application_list", get(application_list_handler))
        .with_state(shared_state.clone());

    if !SETTINGS.read().await.get_string("main.secret").unwrap_or_else(|_| "".to_string()).is_empty() {
        info!("Authentication enabled");
        router = router.layer(from_fn_with_state(shared_state, middleware::authentication_middleware));
    }

    // These routes are excluded from authentication
    router = router
        .route("/assets/*file", get(static_handler))
        .fallback(index_handler);

    router
}