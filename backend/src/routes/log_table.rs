use std::{sync::Arc, str::FromStr, fmt};

use anyhow::Result;
use log::{error, trace};
use serde::{Deserialize, Deserializer, de};
use axum::{Extension, extract::Query, Json};
use axum::http::StatusCode;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use time::OffsetDateTime;
use tokio::sync::RwLock;

use crate::LogEntry;

#[derive(Debug, Deserialize)]
pub struct Params {
    index: usize,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    min_log_level: Option<log::LevelFilter>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    module_name: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    message: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    start_timestamp: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    end_timestamp: Option<String>,
}

#[derive(Debug)]
struct LogFilter {
    index: usize,
    min_log_level: Option<log::LevelFilter>,
    module_name: Option<regex::Regex>,
    message: Option<regex::Regex>,
    start_timestamp: Option<OffsetDateTime>,
    end_timestamp: Option<OffsetDateTime>,
}

impl LogFilter {
    fn new(params: &Params) -> Result<Self> {
        let Params {
            index,
            min_log_level,
            module_name,
            message,
            start_timestamp,
            end_timestamp,
        } = params;

        let index = *index;

        let min_log_level = *min_log_level;

        let module_name = module_name.as_ref().map(|module_name| {
            regex::Regex::new(module_name)
        }).transpose()?;

        let message = message.as_ref().map(|message| {
            regex::Regex::new(message)
        }).transpose()?;

        let start_timestamp = start_timestamp.as_ref().map(|start_timestamp| {
            OffsetDateTime::parse(start_timestamp, &time::format_description::well_known::Iso8601::DEFAULT)
        }).transpose()?;

        let end_timestamp = end_timestamp.as_ref().map(|end_timestamp| {
            OffsetDateTime::parse(end_timestamp, &time::format_description::well_known::Iso8601::DEFAULT)
        }).transpose()?;

        Ok(Self {
            index,
            min_log_level,
            module_name,
            message,
            start_timestamp,
            end_timestamp,
        })
    }
    fn matches(&self, entry: &LogEntry) -> bool {
        if let Some(min_log_level) = self.min_log_level {
            if entry.level > min_log_level {
                return false;
            }
        }

        if let Some(module_name) = &self.module_name {
            if !module_name.is_match(&entry.module) {
                return false;
            }
        }

        if let Some(message) = &self.message {
            if !message.is_match(&entry.message) {
                return false;
            }
        }

        if let Some(start_timestamp) = &self.start_timestamp {
            if entry.timestamp < *start_timestamp {
                return false;
            }
        }

        if let Some(end_timestamp) = &self.end_timestamp {
            if entry.timestamp > *end_timestamp {
                return false;
            }
        }

        true
    }
}

pub async fn log_table_handler(Query(params): Query<Params>, Extension(log_array): Extension<Arc<RwLock<AllocRingBuffer<LogEntry>>>>) -> (StatusCode, Json<Vec<LogEntry>>) {
    trace!("Request received {:?}", params);

    let log_filter_result = LogFilter::new(&params);
    if let Err(err) = log_filter_result {
        error!("Error parsing log filter: {} with params: {:?}", err, params);
        return (StatusCode::BAD_REQUEST, Json(vec![]));
    }

    let log_filter = log_filter_result.unwrap(); // Safe to unwrap because we checked for errors above

    let log_array = log_array.read().await;
    let result = log_array.iter()
        .skip(log_filter.index)
        .filter(|entry| log_filter.matches(entry))
        .take(25)
        .cloned()
        .collect::<Vec<LogEntry>>();

    (StatusCode::OK, Json(result))
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