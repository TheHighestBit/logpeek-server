use axum::middleware::from_fn_with_state;
use axum::{routing::get, Router};
use dashboard_info::dashboard_info_handler;
use log::info;
use log_table::log_table_handler;
use memory_serve::{load_assets, MemoryServe};
use sysinfo::sysinfo_handler;

use crate::routes::application_list::application_list_handler;
use crate::routes::authenticate::authenticate_handler;
use crate::{middleware, SharedState, SETTINGS};

mod application_list;
mod authenticate;
mod dashboard_info;
mod log_table;
mod sysinfo;

pub async fn router_setup(shared_state: SharedState) -> Router {
    let mut router = Router::new()
        .route("/api/dashboard_info", get(dashboard_info_handler))
        .route("/api/log_table", get(log_table_handler))
        .route("/api/authenticate", get(authenticate_handler))
        .layer(from_fn_with_state(
            shared_state.clone(),
            middleware::buffer_refresh_middleware,
        ))
        .route("/api/sysinfo", get(sysinfo_handler))
        .route("/api/application_list", get(application_list_handler))
        .with_state(shared_state.clone());

    if !SETTINGS
        .get_string("main.secret")
        .unwrap_or_else(|_| "".to_string())
        .is_empty()
    {
        info!("Authentication enabled");
        router = router.layer(from_fn_with_state(
            shared_state,
            middleware::authentication_middleware,
        ));
    }

    let static_router = MemoryServe::new(load_assets!("src/dist"))
        .index_file(Some("/index.html"))
        .fallback(Some("/index.html"))
        .into_router();

    router = router.merge(static_router);

    router
}
