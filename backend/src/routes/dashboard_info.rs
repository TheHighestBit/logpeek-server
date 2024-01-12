use std::sync::Arc;
use axum::{Extension, Json};
use log::trace;
use ringbuffer::AllocRingBuffer;
use ringbuffer::RingBuffer;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::LogEntry;

#[derive(Serialize)]
pub struct DashboardResponse {
    total_logs: [u32; 24],
    error_logs: [u32; 24],
    warning_logs: [u32; 24],
    log_buffer_usage: f32,
}

pub async fn dashboard_info_handler(Extension(log_buffer): Extension<Arc<RwLock<AllocRingBuffer<LogEntry>>>>) -> Json<DashboardResponse> {
    trace!("Request received");
    
    let log_array = log_buffer.read().await;
    let current_time = time::OffsetDateTime::now_utc();
    let mut total_logs: [u32; 24] = [0; 24];
    let mut error_logs: [u32; 24] = [0; 24];
    let mut warning_logs: [u32; 24] = [0; 24];

    for entry in log_array.iter().rev().take_while(|entry| entry.timestamp > current_time - time::Duration::hours(24)) {
        let hour: usize = (current_time - entry.timestamp).whole_hours() as usize;
        match entry.level {
            log::Level::Error => error_logs[hour] += 1,
            log::Level::Warn => warning_logs[hour] += 1,
            _ => {}
        }

        total_logs[hour] += 1;
    }

    Json(DashboardResponse {
        total_logs,
        error_logs,
        warning_logs,
        log_buffer_usage: log_array.len() as f32 / log_array.capacity() as f32,
    })
}