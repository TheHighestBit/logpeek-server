mod parsers;

use anyhow::Result;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::{fs::File, io::BufRead};
use std::fs::metadata;
use log::error;
use std::sync::Arc;
use glob::glob;
use rev_lines::RevLines;
use tokio::sync::RwLock;
use crate::LogEntry;

pub fn load_logs() -> Result<Arc<RwLock<AllocRingBuffer<LogEntry>>>> {
    let mut result = AllocRingBuffer::new(15_000_000);
    let mut log_files = Vec::new();

    // Current issue is that all log files need to be processed regardless of modified time
    // Ideally would start reading from the newest and go until the buffer is full.
    for path in glob("logpeek-logs/**/*.log").expect("Failed to read glob pattern") {
        match path {
            Ok(path) => {
                log_files.push((path.clone(), get_modified_time(path.to_str().unwrap())));
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
                    match parsers::parse_logpeek(&line) {
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