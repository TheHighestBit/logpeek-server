mod parser;


use ringbuffer::{AllocRingBuffer, RingBuffer};
use time::format_description::{self, FormatItem};
use std::fs::File;
use std::collections::HashMap;
use std::fs::metadata;
use std::io::{BufRead, BufReader};
use log::{error, trace, warn};
use std::sync::Arc;
use config::{Value, ValueKind};
use glob::glob;
use regex::Regex;

use tokio::sync::{Mutex, RwLock};
use crate::LogEntry;
use crate::SETTINGS;

pub enum TimeFormat<'a> {
    Iso8601,
    Rfc3339,
    Rfc2822,
    Custom(Vec<FormatItem<'a>>), 
}

pub async fn load_logs(buffer: Arc<RwLock<AllocRingBuffer<LogEntry>>>, cache: Arc<Mutex<HashMap<String, (std::time::SystemTime, usize)>>>, i_to_app: Arc<Mutex<HashMap<usize, String>>>) {
    let mut log_buffer = buffer.write().await;
    let mut log_files = Vec::new();
    let mut cache = cache.lock().await;
    let mut i_to_app = i_to_app.lock().await;
    let mut app_i;

    let apps = SETTINGS.read().await.get_array("application").unwrap_or_else(|_| vec![Value::new(None, create_default_map())]);
    
    for app in apps {
        let app_table = app.into_table().expect("Config file is formatted incorrectly!");

        let app_path = app_table
            .get("path")
            .expect("An application is missing the required path in the config!")
            .clone()
            .into_string()
            .expect("Path is not a string!");
        
        if !i_to_app.values().any(|app_name| app_name == &app_path) {
            let length = i_to_app.len();
            app_i = length;
            i_to_app.insert(length, app_path.clone());
        } else {
            app_i = *i_to_app.iter().find(|(_, app_name)| app_name == &&app_path).unwrap().0;
        }

        let app_parser = Regex::new(&app_table
            .get("parser")
            .expect("An application is missing the parser field in the config!")
            .clone()
            .into_string()
            .expect("Parser is not a string!")).expect("Failed to compile regex!");


        let configured_timeformat = app_table
            .get("timeformat")
            .expect("An application is missing the timeformat field in the config!")
            .clone()
            .into_string()
            .expect("Timeformat is not a string!");
        

        let app_timeformat = match configured_timeformat.as_str() {
        "iso8601" => TimeFormat::Iso8601,
        "rfc3339" => TimeFormat::Rfc3339,
        "rfc2822" => TimeFormat::Rfc2822,
        custom_format_str => {
            let format_desc = format_description::parse_borrowed::<1>(custom_format_str).expect("Invalid custom time format!");
            TimeFormat::Custom(format_desc)
        },
        };


        for log_file in glob(format!("{}/*.log", app_path).as_str()).expect("Failed to read glob pattern") {
            match log_file {
                Ok(log_file) => {
                    log_files.push((log_file.clone(), get_modified_time(log_file.to_str().unwrap())));
                },
                Err(e) => error!("Failed to read log file! {:?}", e),
            }
        }

        // Filter out files that haven't been modified since the last time we read them
        log_files.retain(|file| {
            let (path, modified_time) = file;
            match cache.get(path.to_str().unwrap()) {
                Some((cache_time, _)) => modified_time > cache_time,
                None => true,
            }
        });
        log_files.sort_by(|a, b| b.1.cmp(&a.1));

        for log_file in log_files.iter().rev() {
            let file = File::open(log_file.0.clone()).expect("Failed to open file");
            let reader = BufReader::new(file);
            let mut line_count = 0;

            let lines_to_skip = match cache.get(log_file.0.to_str().unwrap()) {
                Some(cached_value) => cached_value.1,
                None => 0,
            };

            for (i, line) in reader.lines().skip(lines_to_skip).enumerate() {
                match line {
                    Ok(line) => {
                        match parser::parse_entry(&line, &app_parser, &app_timeformat, app_i) {
                            Ok(parse_result) => {
                                trace!("{:?}", parse_result);
                                log_buffer.push(parse_result);
                            },
                            Err(err) => {
                                error!("{} on line {} in file {}", err, i + 1, log_file.0.to_str().unwrap());
                            }
                        }
                    },
                    Err(err) => {
                        error!("{}", err);
                    }
                }

                line_count = i;
            }

            cache.insert(log_file.0.to_str().unwrap().to_string(), (log_file.1, line_count));
        }

        log_files.clear();
    }
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
    warn!("No application configurations found, proceeding with defaults.");
    
    let mut map = HashMap::new();
    map.insert("path".to_string(), Value::new(None, ValueKind::String("logpeek-logs".to_string())));
    map.insert("parser".to_string(), Value::new(None, ValueKind::String(r"^(?P<timestamp>\S+) (?P<level>\S+) (?P<module>\S+) - (?P<message>.+)$".to_string())));
    map.insert("timeformat".to_string(), Value::new(None, ValueKind::String("iso8601".to_string())));

    ValueKind::Table(map)
}