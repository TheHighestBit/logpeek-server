use config::{Value, ValueKind};
use glob::glob;
use log::{debug, error, trace, warn};
use regex::Regex;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::metadata;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::System;
use time::format_description::{self, FormatItem};
use tokio::sync::{Mutex, RwLock};

use crate::LogEntry;
use crate::SETTINGS;

mod parser;

pub enum TimeFormat<'a> {
    Iso8601,
    Rfc3339,
    Rfc2822,
    Custom(Vec<FormatItem<'a>>),
}

pub async fn load_logs(
    buffer: Arc<RwLock<HashMap<usize, AllocRingBuffer<LogEntry>>>>,
    cache: Arc<Mutex<HashMap<String, (std::time::SystemTime, usize)>>>,
    i_to_app: Arc<Mutex<HashMap<usize, String>>>,
    sysinfo: Arc<Mutex<System>>,
    is_init: bool,
) {
    let mut log_buffer_map = buffer.write().await;
    let mut log_files = Vec::new();
    let mut cache = cache.lock().await;
    let mut i_to_app = i_to_app.lock().await;
    let mut app_i;

    let apps = SETTINGS
        .get_array("application")
        .unwrap_or_else(|_| vec![Value::new(None, create_default_map())]);

    for app in apps {
        let app_table = app
            .into_table()
            .expect("Config file is formatted incorrectly!");

        let app_path = app_table
            .get("path")
            .expect("An application is missing the required path in the config!")
            .clone()
            .into_string()
            .expect("Path is not a string!");

        let app_name = app_table
            .get("name")
            .map(|name| name.clone().into_string().expect("Name is not a string!"))
            .unwrap_or_else(|| app_path.clone());

        let level_map: Option<HashMap<String, String>> = app_table
            .get("level_map")
            .and_then(|level_map| level_map.clone().into_table().ok())
            .map(|table| {
                table
                    .into_iter()
                    .filter_map(|(k, v)| v.into_string().ok().map(|val| (k, val)))
                    .collect()
            });

        if !i_to_app
            .values()
            .any(|stored_app_name| stored_app_name == &app_name)
        {
            let length = i_to_app.len();
            app_i = length;
            i_to_app.insert(length, app_name.clone());
        } else {
            app_i = *i_to_app
                .iter()
                .find(|(_, stored_app_name)| stored_app_name == &&app_name)
                .unwrap()
                .0;
        }

        let app_parser = Regex::new(
            &app_table
                .get("parser")
                .expect("An application is missing the parser field in the config!")
                .clone()
                .into_string()
                .expect("Parser is not a string!"),
        )
        .expect("Failed to compile regex!");

        let configured_timeformat = if let Some(configured_timeformat) = app_table.get("timeformat")
        {
            configured_timeformat.clone().to_string()
        } else {
            "iso8601".to_string()
        };

        let app_timeformat = match configured_timeformat.as_str() {
            "iso8601" => TimeFormat::Iso8601,
            "rfc3339" => TimeFormat::Rfc3339,
            "rfc2822" => TimeFormat::Rfc2822,
            custom_format_str => {
                let format_desc = format_description::parse_borrowed::<1>(custom_format_str)
                    .expect("Invalid custom time format!");
                TimeFormat::Custom(format_desc)
            }
        };

        debug!("Loading logs for application: {}", app_path);

        if metadata(&app_path)
            .expect("Failed to read metadata for log file")
            .is_file()
        {
            log_files.push((
                PathBuf::from(app_path.clone()),
                get_modified_time(&app_path),
            ));
        } else {
            for log_file in
                glob(format!("{}/**/*", &app_path).as_str()).expect("Failed to read glob pattern")
            {
                if let Ok(path) = log_file {
                    if let Ok(metadata) = metadata(&path) {
                        if metadata.is_file() {
                            log_files
                                .push((path.clone(), get_modified_time(path.to_str().unwrap())));
                        }
                    } else {
                        error!("Failed to read log file metadata! {:?}", path);
                    }
                } else {
                    error!("Failed to read log file! {:?}", log_file);
                }
            }
        }

        let log_buffer = match log_buffer_map.entry(app_i) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let app_buffer_size = app_table
                    .get("buffer_size")
                    .unwrap_or(&Value::new(None, ValueKind::U64(1_000_000)))
                    .clone()
                    .into_uint()
                    .expect("buffer_size is not parsable to an unsigned integer!");

                // This memory check is expensive, but it is only done once during initialization
                let memory_required = get_memory_required::<LogEntry>(app_buffer_size);
                let mut sys = sysinfo.lock().await;
                let available_memory = get_available_memory(&mut sys);

                if available_memory < memory_required {
                    panic!("{} MB is required to allocate buffer for {}, which is more than the currently available memory of {} MB!",
                        memory_required / 1_048_576, &app_path, available_memory / 1_048_576);
                } else if available_memory < memory_required * 2 {
                    warn!("{} MB is required to allocate buffer for {}, which is more than half of the currently available memory! \
                    Consider reducing the buffer sizes to avoid runtime out-of-memory issues.", memory_required / 1_048_576, &app_path);
                } else {
                    debug!(
                        "{} MB is required for application {}",
                        memory_required / 1_048_576,
                        &app_path
                    );
                }

                entry.insert(AllocRingBuffer::new(app_buffer_size as usize))
            }
        };

        log_files.sort_by(|a, b| b.1.cmp(&a.1)); // Newest files first

        if is_init {
            // During first load we need to first exclude files that would fall outside the buffer.
            // These files will be inserted into the cache and not processed.

            let mut total_line_count: usize = 0;
            let buffer_size = log_buffer.capacity();
            let mut file_iterator = log_files.iter(); // Starting from the newest file

            while total_line_count < buffer_size {
                let log_file = match file_iterator.next() {
                    Some(file) => file,
                    None => break,
                };

                debug!(
                    "Counting lines in log file: {}",
                    log_file.0.to_str().unwrap()
                );

                let file_line_count =
                    BufReader::new(File::open(log_file.0.clone()).expect("Failed to open file"))
                        .lines()
                        .count();
                if total_line_count + file_line_count >= buffer_size {
                    // This is the earliest file we need to read from
                    // Bump the cache time so that the following retain call will include it
                    cache.insert(
                        log_file.0.to_str().unwrap().to_string(),
                        (
                            log_file.1.checked_sub(Duration::from_secs(1)).unwrap(),
                            file_line_count - (buffer_size - total_line_count),
                        ),
                    );
                    break;
                } else {
                    total_line_count += file_line_count;
                }
            }

            // Add the rest to cache
            for log_file in file_iterator {
                cache.insert(log_file.0.to_str().unwrap().to_string(), (log_file.1, 0));
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

        for log_file in log_files.iter().rev() {
            debug!("Reading log file: {}", log_file.0.to_str().unwrap());
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
                        match parser::parse_entry(
                            &line,
                            &app_parser,
                            &app_timeformat,
                            app_i,
                            &level_map,
                        ) {
                            Ok(parse_result) => {
                                trace!("{:?}", parse_result);
                                log_buffer.push(parse_result);
                            }
                            Err(err) => {
                                error!(
                                    "{} on line {} in file {}",
                                    err,
                                    i + 1,
                                    log_file.0.to_str().unwrap()
                                );
                            }
                        }
                    }
                    Err(err) => {
                        error!("{}", err);
                    }
                }

                line_count = i;
            }

            cache.insert(
                log_file.0.to_str().unwrap().to_string(),
                (log_file.1, line_count),
            );
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
        }
    }
}

fn create_default_map() -> ValueKind {
    warn!("No application configurations found, proceeding with defaults.");

    let mut map = HashMap::new();
    map.insert(
        "path".to_string(),
        Value::new(None, ValueKind::String("logpeek-logs".to_string())),
    );
    map.insert(
        "parser".to_string(),
        Value::new(
            None,
            ValueKind::String(
                r"^(?P<timestamp>\S+) (?P<level>\S+) (?P<module>\S+) - (?P<message>.+)$"
                    .to_string(),
            ),
        ),
    );
    map.insert(
        "timeformat".to_string(),
        Value::new(None, ValueKind::String("iso8601".to_string())),
    );
    map.insert(
        "buffer_size".to_string(),
        Value::new(None, ValueKind::U64(1_000_000)),
    );

    ValueKind::Table(map)
}

fn get_available_memory(sysinfo: &mut System) -> u64 {
    sysinfo.refresh_memory();
    sysinfo.total_memory() - sysinfo.used_memory()
}

fn get_memory_required<T>(buffer_size: u64) -> u64 {
    buffer_size * size_of::<T>() as u64
}
