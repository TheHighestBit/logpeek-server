use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use log::warn;
use regex::Regex;
use thiserror::Error;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

use crate::log_reader::TimeFormat;
use crate::LogEntry;

#[derive(Debug, Error)]
pub enum LogParseError {
    #[error("No capture groups found in line: {0}")]
    NoCaptureGroupsFound(String),
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
}

pub fn parse_entry(
    line: &str,
    parser_re: &Regex,
    timeformat: &TimeFormat,
    app_i: usize,
    level_map: &Option<HashMap<String, String>>,
) -> Result<LogEntry> {
    if let Some(caps) = parser_re.captures(line) {
        let timestamp = if let Some(timestamp) = caps.name("timestamp") {
            match timeformat {
                TimeFormat::Iso8601 => {
                    OffsetDateTime::parse(timestamp.as_str(), &Iso8601::DEFAULT)?
                }
                TimeFormat::Rfc2822 => OffsetDateTime::parse(
                    timestamp.as_str(),
                    &time::format_description::well_known::Rfc2822,
                )?,
                TimeFormat::Rfc3339 => OffsetDateTime::parse(
                    timestamp.as_str(),
                    &time::format_description::well_known::Rfc3339,
                )?,
                TimeFormat::Custom(format_desc) => {
                    OffsetDateTime::parse(timestamp.as_str(), &format_desc)?
                }
            }
        } else {
            OffsetDateTime::now_utc()
        };

        let level = caps
            .name("level")
            .and_then(|level| {
                let level_str = level.as_str();

                level_map
                    .as_ref()
                    .and_then(|map| map.get(level_str).cloned())
                    .map(|mapped_level| {
                        log::Level::from_str(&mapped_level).unwrap_or_else(|_| {
                            warn!(
                                "Invalid log level mapping: {}. Using INFO instead",
                                mapped_level
                            );
                            log::Level::Info
                        })
                    })
                    .or_else(|| {
                        let from_str_result = log::Level::from_str(level_str);
                        
                        match from_str_result {
                            Ok(level) => Some(level),
                            Err(_) => {
                                warn!("Invalid log level: {}. Consider adding a mapping for it under the applications level_map field.", level_str);
                                None
                            }
                        }
                    })
            })
            .unwrap_or(log::Level::Info);

        let module = if let Some(module) = caps.name("module") {
            module.as_str()
        } else {
            "N/A"
        };

        let message = caps
            .name("message")
            .ok_or_else(|| LogParseError::InvalidMessage(line.to_string()))?
            .as_str(); // Is required

        Ok(LogEntry {
            timestamp,
            level,
            module: module.to_string(),
            message: message.to_string(),
            application: app_i,
        })
    } else {
        Err(LogParseError::NoCaptureGroupsFound(line.to_string()).into())
    }
}
