mod routes;
mod log_reader;

use ringbuffer::RingBuffer;
use serde::Serialize;
use std::sync::Arc;
use time::OffsetDateTime;
use axum::Router;
use log::info;
use routes::router_setup;


#[derive(Debug, Serialize, Clone)]
struct LogEntry {
    timestamp: OffsetDateTime,
    level: log::Level,
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
    info!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}