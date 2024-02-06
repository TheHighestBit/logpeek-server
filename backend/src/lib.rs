mod routes;
mod log_reader;

use ringbuffer::{AllocRingBuffer, RingBuffer};
use serde::Serialize;
use std::sync::Arc;
use time::OffsetDateTime;
use axum::Router;
use config::Config;
use lazy_static::lazy_static;
use log::{info, LevelFilter};
use logpeek::config::LoggingMode;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, System};
use tokio::sync::{Mutex, RwLock};
use routes::router_setup;

#[derive(Debug, Serialize, Clone)]
struct LogEntry {
    #[serde(with = "time::serde::rfc3339")]
    timestamp: OffsetDateTime,
    level: log::Level,
    module: String,
    message: String,
}

#[derive(Clone)]
struct SharedState {
    log_buffer: Arc<RwLock<AllocRingBuffer<LogEntry>>>,
    sys: Arc<Mutex<System>>,
    os: String,
    host_name: String,
}

// Environment overrides the config file
lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .add_source(config::Environment::with_prefix("LOGPEEK"))
        .build()
        .expect("There is an issue with the configuration file"));
}

pub async fn run() {
    // Logger setup
    let logger_config = logpeek::config::Config {
        datetime_format: logpeek::config::DateTimeFormat::ISO8601,
        min_log_level: match SETTINGS.read().await.get_bool("main.logger.debug").unwrap_or(false) {
            true => LevelFilter::Trace,
            false => LevelFilter::Info
        },
        out_dir_name: logpeek::config::OutputDirName::Custom(SETTINGS.read().await.get_string("main.logger.log_dir").unwrap_or("logpeek-logs".to_string())),
        logging_mode: match SETTINGS.read().await.get_bool("main.logger.log_to_file").unwrap_or(true) {
            true => LoggingMode::FileAndConsole,
            false => LoggingMode::Console
        },
        ..Default::default() };
    logpeek::init(logger_config).unwrap();

    info!("Starting...");

    //Read in and process the log entries
    let log_entries = log_reader::load_logs().await.map_err(|err| {
        panic!("Error loading logs: {}", err)
    }).unwrap();
    info!("Loaded {} log entries", log_entries.read().await.len());

    // Initialize the system info
    let sys = Arc::new(Mutex::new(System::new_with_specifics(
        sysinfo::RefreshKind::new()
            .with_cpu(CpuRefreshKind::new().with_cpu_usage())
            .with_memory(MemoryRefreshKind::new().with_ram())
    )));
    let os = System::long_os_version().unwrap_or_default();
    let host_name = System::host_name().unwrap_or_default();

    let shared_state = SharedState {
        log_buffer: log_entries,
        sys,
        os,
        host_name
    };

    let app: Router = router_setup(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    info!("Listening on http://0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}