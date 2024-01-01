use axum::Json;
use log::trace;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MirrorJson {
    message: String
}

pub async fn mirror_json(Json(body): Json<MirrorJson>) -> Json<MirrorJson> {
    trace!("mirror_json request received");
    Json(body)
}