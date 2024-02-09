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
    #[error("Invalid log level: {0}")]
    InvalidLogLevel(String),
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
}

pub fn parse_entry(line: &str, parser_re: &Regex, timeformat: &TimeFormat) -> Result<LogEntry> {
    if let Some(caps) = parser_re.captures(line) {
        let timestamp = caps.name("timestamp").ok_or_else(|| LogParseError::InvalidTimestamp(line.to_string()))?.as_str();
        let level = caps.name("level").ok_or_else(|| LogParseError::InvalidLogLevel(line.to_string()))?.as_str();
        let module = caps.name("module").ok_or_else(|| LogParseError::InvalidMessage(line.to_string()))?.as_str();
        let message = caps.name("message").ok_or_else(|| LogParseError::InvalidMessage(line.to_string()))?.as_str();

        Ok(LogEntry {
            timestamp: match timeformat {
                TimeFormat::Iso8601 => OffsetDateTime::parse(timestamp, &Iso8601::DEFAULT)?,
                TimeFormat::Rfc2822 => OffsetDateTime::parse(timestamp, &time::format_description::well_known::Rfc2822)?,
                TimeFormat::Rfc3339 => OffsetDateTime::parse(timestamp, &time::format_description::well_known::Rfc3339)?
            },
            level: log::Level::from_str(level)?,
            module: module.to_string(),
            message: message.to_string(),
        })
    } else {
        Err(LogParseError::NoCaptureGroupsFound(line.to_string()).into())
    }
}