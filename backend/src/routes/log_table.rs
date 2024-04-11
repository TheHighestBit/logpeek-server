use std::{str::FromStr, fmt};

use anyhow::Result;
use log::{error, trace};
use serde::{Deserialize, Deserializer, de, Serialize};
use axum::Json;
use axum::extract::State;
use axum_extra::extract::Query;
use axum::http::StatusCode;
use time::OffsetDateTime;


use crate::{convert_app_to_i, LogBufferIterator, LogEntry, SharedState};

#[derive(Debug, Deserialize)]
pub struct Params {
    page: usize,
    items_per_page: usize,
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
    applications: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct LogEntryWithApplication {
    pub entry: LogEntry,
    pub application: String,
}

#[derive(Debug, Serialize)]
pub struct LogTableResponse {
    pub total_items: usize,
    pub logs: Vec<LogEntryWithApplication>,
}

#[derive(Debug)]
struct LogFilter {
    index: usize,
    items_per_page: usize,
    min_log_level: Option<log::LevelFilter>,
    module_name: Option<regex::Regex>,
    message: Option<regex::Regex>,
    start_timestamp: Option<OffsetDateTime>,
    end_timestamp: Option<OffsetDateTime>,
}

impl LogFilter {
    async fn new(params: Params) -> Result<Self> {
        let Params {
            page,
            items_per_page,
            min_log_level,
            module_name,
            message,
            start_timestamp,
            end_timestamp,
            applications: _,
        } = params;

        let index = (page - 1) * items_per_page;

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
            items_per_page,
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

pub async fn log_table_handler(Query(params): Query<Params>, State(shared_state): State<SharedState>) -> (StatusCode, Json<LogTableResponse>) {
    trace!("Request received {:?}", &params);

    let i_to_app = shared_state.i_to_app.lock().await;
    let applications = if let Some(param_applications) = &params.applications {
        convert_app_to_i(param_applications, &i_to_app)
    } else {
        Vec::new()
    };

    let log_filter_result = LogFilter::new(params).await;

    match log_filter_result {
        Ok(log_filter) => {
            let log_buffer_map = shared_state.log_buffer.read().await;
            let buffer_iterator = LogBufferIterator::new(&log_buffer_map, &applications);
            
            let mut result: Vec<LogEntryWithApplication> = Vec::new();
            let mut skipped: usize = 0;
            let mut taken: usize = 0;
            let mut total_items: usize = 0;

            for entry in buffer_iterator {
                if log_filter.matches(entry) {
                    total_items += 1;

                    if skipped < log_filter.index {
                        skipped += 1;
                    } else if taken < log_filter.items_per_page {
                        result.push(LogEntryWithApplication {
                            entry: entry.clone(),
                            application: i_to_app.get(&entry.application).unwrap().clone(),
                        });
                        taken += 1;
                    }
                }
            }

            (StatusCode::OK, Json({
                LogTableResponse {
                    total_items,
                    logs: result,
                }
            }))
        },
        Err(err) => {
            error!("Error parsing log filter: {}", err);
            (StatusCode::BAD_REQUEST, Json({
                LogTableResponse {
                    total_items: 0,
                    logs: vec![],
                }
            }))
        }
    }
}

// Serde deserialization decorator to map empty Strings to None,
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
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