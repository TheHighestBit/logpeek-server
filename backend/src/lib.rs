mod routes;
mod log_reader;

use std::sync::Arc;

use axum::Router;
use log::info;
use routes::router_setup;

#[derive(Debug)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    level: LogLevel,
    module: String,
    message: String,
}

pub async fn run() {
    //Read in and process the log entries
    let log_entries = log_reader::load_logs().map_err(|err| {
        panic!("Error loading logs: {}", err)
    }).unwrap();

    info!("Loaded {} log entries", log_entries.read().await.len());

    let app: Router = router_setup(Arc::clone(&log_entries));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}