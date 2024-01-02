use axum::{Router, routing::{get, post}};
use index_handler::index_handler;

use self::index_handler::static_handler;

mod index_handler;
mod mirror_json;

pub fn router_setup() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/assets/*file", get(static_handler))
}