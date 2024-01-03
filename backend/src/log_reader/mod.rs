mod parsers;

use anyhow::Result;
use std::{fs::File, io::BufRead};
use log::error;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::LogEntry;

pub fn load_logs() -> Result<Arc<RwLock<Vec<LogEntry>>>> {
    let mut result = Vec::new();

    let file = File::open("logs/test.log")?;
    let reader = std::io::BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                match parsers::parse_logpeek_iso8601(&line) {
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