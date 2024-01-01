use axum::{Router, routing::{get, post}};
use hello_world::hello_world;

use self::mirror_json::mirror_json;

mod hello_world;
mod mirror_json;

pub fn router_setup() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_json", post(mirror_json))
}