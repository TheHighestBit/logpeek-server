mod parsers;

use anyhow::Result;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::{fs::File, io::BufRead};
use log::error;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::LogEntry;

pub fn load_logs() -> Result<Arc<RwLock<AllocRingBuffer<LogEntry>>>> {
    let mut result = AllocRingBuffer::new(15_000_000);
    let file = File::open("logs/test.log")?;
    let reader = std::io::BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                match parsers::parse_logpeek(&line) {
                    Ok(parse_result) => result.push(parse_result),
                    Err(err) => {
                        error!("{} on line {}", err, i + 1);
                    }
                }
            },
            Err(err) => {
                error!("{}", err);
            }
        }
    }

    Ok(Arc::new(RwLock::new(result)))
}