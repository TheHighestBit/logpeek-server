use regex::Regex;
use anyhow::Result;
use thiserror::Error;
use lazy_static::lazy_static;
use crate::LogLevel;
use crate::LogEntry;

impl LogLevel {
    pub fn from_str(level: &str) -> Result<LogLevel> {
        match level {
            "TRACE" => Ok(LogLevel::Trace),
            "DEBUG" => Ok(LogLevel::Debug),
            "INFO" => Ok(LogLevel::Info),
            "WARN" => Ok(LogLevel::Warn),
            "ERROR" => Ok(LogLevel::Error),
            _ => Err(LogParseError::InvalidLogLevel(level.to_string()).into()),
        }
    }

}

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

lazy_static! {
    static ref LOGPEEK_ISO8601: Regex = Regex::new(r"^(?P<timestamp>\S+) (?P<level>\S+) (?P<module>\S+) - (?P<message>.+)$").unwrap();
}

pub fn parse_logpeek_iso8601(line: &str) -> Result<LogEntry> {
    if let Some(caps) = LOGPEEK_ISO8601.captures(line) {
        let timestamp = caps.name("timestamp").ok_or_else(|| LogParseError::InvalidTimestamp(line.to_string()))?.as_str();
        let level = caps.name("level").ok_or_else(|| LogParseError::InvalidLogLevel(line.to_string()))?.as_str();
        let module = caps.name("module").ok_or_else(|| LogParseError::InvalidMessage(line.to_string()))?.as_str();
        let message = caps.name("message").ok_or_else(|| LogParseError::InvalidMessage(line.to_string()))?.as_str();

        Ok(LogEntry {
            timestamp: timestamp.to_string(),
            level: LogLevel::from_str(level)?,
            module: module.to_string(),
            message: message.to_string(),
        })
    } else {
        Err(LogParseError::NoCaptureGroupsFound(line.to_string()).into())
    }
}