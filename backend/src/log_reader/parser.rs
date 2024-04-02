use std::str::FromStr;

use regex::Regex;
use anyhow::Result;
use thiserror::Error;
use time::OffsetDateTime;
use time::format_description::well_known::Iso8601;
use crate::log_reader::TimeFormat;
use crate::LogEntry;

#[derive(Debug, Error)]
pub enum LogParseError {
    #[error("No capture groups found in line: {0}")]
    NoCaptureGroupsFound(String),
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
}

pub fn parse_entry(line: &str, parser_re: &Regex, timeformat: &TimeFormat, app_i: usize) -> Result<LogEntry> {
    if let Some(caps) = parser_re.captures(line) {
        let timestamp = if let Some(timestamp) = caps.name("timestamp") {
            timestamp.as_str()
        } else {
            "2000-01-01T00:00:00.000Z"
        };

        let level = if let Some(level) = caps.name("level") {
            level.as_str()
        } else {
            "INFO"
        };

        let module = if let Some(module) = caps.name("module") {
            module.as_str()
        } else {
            "N/A"
        };

        let message = caps.name("message").ok_or_else(|| LogParseError::InvalidMessage(line.to_string()))?.as_str(); // Is required

        Ok(LogEntry {
            timestamp: match timeformat {
                TimeFormat::Iso8601 => OffsetDateTime::parse(timestamp, &Iso8601::DEFAULT)?,
                TimeFormat::Rfc2822 => OffsetDateTime::parse(timestamp, &time::format_description::well_known::Rfc2822)?,
                TimeFormat::Rfc3339 => OffsetDateTime::parse(timestamp, &time::format_description::well_known::Rfc3339)?,
                TimeFormat::Custom(format_desc) => OffsetDateTime::parse(timestamp, &format_desc)?,
            },
            level: log::Level::from_str(level)?,
            module: module.to_string(),
            message: message.to_string(),
            application: app_i
        })
    } else {
        Err(LogParseError::NoCaptureGroupsFound(line.to_string()).into())
    }
}