use std::{sync::Arc, str::FromStr, fmt};

use log::{info, trace};
use serde::{Deserialize, Deserializer, de, Serialize};
use axum::{Extension, extract::Query, Json};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use tokio::sync::RwLock;

use crate::LogEntry;

#[derive(Debug, Deserialize)]
pub struct Params {
    index: u64,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    filter_level: Option<log::Level>
}

#[derive(Debug, Serialize)]
pub struct Response {
    log_entries: Vec<LogEntry>,
}

pub async fn log_table_handler(Query(params): Query<Params>, Extension(log_array): Extension<Arc<RwLock<AllocRingBuffer<LogEntry>>>>) -> Json<Response> {
    trace!("Request received {:?}", params);

    let result: Vec<LogEntry> = Vec::with_capacity(10);
    let log_array = log_array.read().await;

    let result = log_array.iter().take(10).cloned().collect::<Vec<LogEntry>>();
    
    Json(Response {
        log_entries: result,
    })
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}