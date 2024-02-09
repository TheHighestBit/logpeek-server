mod parser;

use anyhow::Result;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::{fs::File};
use std::collections::HashMap;
use std::fs::metadata;
use log::{error};
use std::sync::Arc;
use config::{Value, ValueKind};
use glob::glob;
use regex::Regex;
use rev_lines::RevLines;
use tokio::sync::RwLock;
use crate::LogEntry;
use crate::SETTINGS;

pub enum TimeFormat {
    Iso8601,
    Rfc3339,
    Rfc2822,
}

pub async fn load_logs() -> Result<Arc<RwLock<AllocRingBuffer<LogEntry>>>> {
    let mut result = AllocRingBuffer::new(SETTINGS.read().await.get_int("main.buffer_size").unwrap_or(1_000_000) as usize);
    let mut log_files = Vec::new();

    let apps = SETTINGS.read().await.get_array("application").unwrap_or(vec![Value::new(None, create_default_map())]);

    for app in apps {
        let app_table = app.into_table().expect("Config file is formatted incorrectly!");

        let app_path = app_table
            .get("path")
            .expect("An application is missing the required path in the config!")
            .clone()
            .into_string()
            .expect("Path is not a string!");

        let app_parser = Regex::new(&app_table
            .get("parser")
            .expect("An application is missing the parser field in the config!")
            .clone()
            .into_string()
            .expect("Parser is not a string!")).expect("Failed to compile regex!");

        let app_timeformat = match app_table
            .get("timeformat")
            .expect("An application is missing the timeformat field in the config!")
            .clone()
            .into_string()
            .expect("Timeformat is not a string!")
            .as_str() {
            "iso8601" => TimeFormat::Iso8601,
            "rfc3339" => TimeFormat::Rfc3339,
            "rfc2822" => TimeFormat::Rfc2822,
            _ => panic!("Invalid timeformat!"),
        };

        for log_file in glob(format!("{}/*.log", app_path).as_str()).expect("Failed to read glob pattern") {
            match log_file {
                Ok(log_file) => {
                    log_files.push((log_file.clone(), get_modified_time(log_file.to_str().unwrap())));
                },
                Err(e) => error!("Failed to read log file! {:?}", e),
            }
        }

        // Sort the log files by modified time
        log_files.sort_by(|a, b| b.1.cmp(&a.1));

        for log_file in log_files.iter() {
            let file = File::open(log_file.0.clone())?;
            let reader = RevLines::new(file);
            for (i, line) in reader.enumerate() {
                match line {
                    Ok(line) => {
                        match parser::parse_entry(&line, &app_parser, &app_timeformat) {
                            Ok(parse_result) => result.push(parse_result),
                            Err(err) => {
                                error!("{} on line {} in file {}", err, i + 1, log_file.0.to_str().unwrap());
                            }
                        }
                    },
                    Err(err) => {
                        error!("{}", err);
                    }
                }
            }
        }

        log_files.clear();
    }

    Ok(Arc::new(RwLock::new(result)))
}

fn get_modified_time(path: &str) -> std::time::SystemTime {
    match metadata(path) {
        Ok(metadata) => metadata.modified().unwrap_or_else(|_| {
            error!("Failed to read modified time for log file!");
            std::time::SystemTime::UNIX_EPOCH
        }),
        Err(e) => {
            error!("Failed to read metadata for log file! {:?}", e);
            std::time::SystemTime::UNIX_EPOCH
        },
    }
}

fn create_default_map() -> ValueKind {
    let mut map = HashMap::new();
    map.insert("path".to_string(), Value::new(None, ValueKind::String("logs".to_string())));
    map.insert("parser".to_string(), Value::new(None, ValueKind::String(r"^(?P<timestamp>\S+) (?P<level>\S+) (?P<module>\S+) - (?P<message>.+)$".to_string())));
    map.insert("timeformat".to_string(), Value::new(None, ValueKind::String("iso8601".to_string())));

    ValueKind::Table(map)
}