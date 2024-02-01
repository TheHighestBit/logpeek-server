mod routes;
mod log_reader;

use ringbuffer::{AllocRingBuffer, RingBuffer};
use serde::Serialize;
use std::sync::Arc;
use time::OffsetDateTime;
use axum::Router;
use log::info;
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

pub async fn run() {
    //Read in and process the log entries
    let log_entries = log_reader::load_logs().map_err(|err| {
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