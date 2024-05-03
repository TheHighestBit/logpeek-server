mod routes;
mod log_reader;
mod middleware;
mod config;

use std::collections::HashMap;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use serde::Serialize;
use std::sync::Arc;
use std::time::SystemTime;
use ::config::Config;
use time::OffsetDateTime;
use axum::Router;
use log::{info, LevelFilter};
use logpeek::config::LoggingMode;
use once_cell::sync::Lazy;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, System};
use tokio::signal;
use tokio::sync::{Mutex, MutexGuard, RwLock};
use routes::router_setup;
use crate::config::config_setup;

#[derive(Debug, Serialize, Clone)]
struct LogEntry {
    #[serde(with = "time::serde::rfc3339")]
    timestamp: OffsetDateTime,
    level: log::Level,
    module: String,
    message: String,
    application: usize,
}

#[derive(Clone)]
struct SharedState {
    log_buffer: Arc<RwLock<HashMap<usize, AllocRingBuffer<LogEntry>>>>,
    cache: Arc<Mutex<HashMap<String, (SystemTime, usize)>>>,
    i_to_app: Arc<Mutex<HashMap<usize, String>>>,
    last_buffer_update: Arc<Mutex<SystemTime>>,
    sys: Arc<Mutex<System>>,
    server_start_time: Arc<SystemTime>,
    os: Arc<String>,
    host_name: Arc<String>,
    login_attempts: Arc<Mutex<u32>>,
}

static SETTINGS: Lazy<Config> = config_setup();

pub async fn run() {
    // Logger setup
    let logger_config = logpeek::config::Config {
        min_log_level: match SETTINGS.get_bool("main.logger.enable_debug").unwrap_or(false) {
            true => LevelFilter::Debug,
            false => LevelFilter::Info
        },
        out_dir_name: logpeek::config::OutputDirName::Custom(SETTINGS.get_string("main.logger.log_path").unwrap_or_else(|_| "logpeek-logs".to_string())),
        logging_mode: match SETTINGS.get_bool("main.logger.log_to_file").unwrap_or(true) {
            true => LoggingMode::FileAndConsole,
            false => LoggingMode::Console
        },
        target_filter: Some(vec!["memory_serve"]),
        ..Default::default() };
    logpeek::init(logger_config).unwrap();

    info!("Starting...");

    //Read in and process the log entries
    let log_buffer = Arc::new(RwLock::new(HashMap::new()));
    let cache = Arc::new(Mutex::new(HashMap::new()));
    let i_to_app = Arc::new(Mutex::new(HashMap::new()));

    // Initialize the system info
    let sys = Arc::new(Mutex::new(System::new_with_specifics(
        sysinfo::RefreshKind::new()
            .with_cpu(CpuRefreshKind::new().with_cpu_usage())
            .with_memory(MemoryRefreshKind::new().with_ram())
    )));
    let os = System::long_os_version().unwrap_or_default();
    let host_name = System::host_name().unwrap_or_default();
    
    let load_start = SystemTime::now();
    log_reader::load_logs(log_buffer.clone(), cache.clone(), i_to_app.clone(), sys.clone(), true).await;
    let loaded_count = log_buffer.read().await.iter().map(|(_, buffer)| buffer.len()).sum::<usize>();
    info!("Loaded {} log entries for {} applications in {:?}", loaded_count, log_buffer.read().await.len(), load_start.elapsed().unwrap());

    let shared_state = SharedState {
        log_buffer,
        cache,
        i_to_app,
        last_buffer_update: Arc::new(Mutex::new(SystemTime::now())),
        sys,
        server_start_time: Arc::new(SystemTime::now()),
        os: Arc::new(os),
        host_name: Arc::new(host_name),
        login_attempts: Arc::new(Mutex::new(0)),
    };

    let host_address = SETTINGS.get_string("main.address").unwrap_or_else(|_| "127.0.0.1:3001".to_string());

    let app: Router = router_setup(shared_state).await;

    let listener = tokio::net::TcpListener::bind(&host_address).await.unwrap();
    
    info!("Listening on http://{}", &host_address);
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_handler()) // Axum's docs say to also add a timeout layer for this but that would require adding tower as a dependency. It seems to be fine anyway...
        .await
        .unwrap();
}

pub fn convert_app_to_i(app: &str, i_to_app: &MutexGuard<HashMap<usize, String>>) -> Option<usize> {
    // Application paths are stored in a hashmap that maps an index to the path. Before filtering, we need to convert back to index representation.
    i_to_app.iter().find(|(_, stored_app_name)| **stored_app_name == app).map(|(i, _)| i).copied()
}

// This iterator yields the most recent log entry across all the buffers
struct LogBufferIterator<'a> {
    buffers: Vec<(&'a AllocRingBuffer<LogEntry>, isize)>,
}

impl<'a> LogBufferIterator<'a> {
    fn new(buffer_map: &'a HashMap<usize, AllocRingBuffer<LogEntry>>, app_filter: Option<usize>) -> Self {
        let buffers = buffer_map.iter()
            .filter(|entry| app_filter.is_none() || app_filter.unwrap() == *entry.0)
            .map(|entry| (entry.1, -1)).collect();

        LogBufferIterator {
            buffers
        }
    }
}

impl<'a> Iterator for LogBufferIterator<'a> {
    type Item = &'a LogEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut latest_time: Option<&OffsetDateTime> = None;
        let mut latest_entry: Option<&LogEntry> = None;
        let mut index: Option<usize> = None;

        for (i, buffer) in self.buffers.iter().enumerate() {
            if let Some(entry) = buffer.0.get_signed(buffer.1) { // -1 is the most recent entry, -2 is the second most recent, etc.
                if let Some(current_latest) = latest_time {
                    if entry.timestamp > *current_latest {
                        latest_time = Some(&entry.timestamp);
                        latest_entry = Some(entry);
                        index = Some(i);
                    }
                } else {
                    latest_time = Some(&entry.timestamp);
                    latest_entry = Some(entry);
                    index = Some(i);
                }
            }
        }

        if let Some(i) = index {
            self.buffers[i].1 -= 1;
            
            if self.buffers[i].1 < -(self.buffers[i].0.len() as isize) {
                self.buffers.remove(i);
            }
        }
        
        latest_entry
    }
}

// Straight from axum examples
async fn shutdown_handler() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Exiting...");
}