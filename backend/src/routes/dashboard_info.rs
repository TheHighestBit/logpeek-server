use std::sync::Arc;

use axum::{Extension, Json};
use log::trace;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::LogEntry;

#[derive(Serialize)]
pub struct DashboardInfo {
    total_count: u32,
    error_count: u32,
    warning_count: u32,
}

pub async fn dashboard_info_handler(Extension(log_array): Extension<Arc<RwLock<Vec<LogEntry>>>>) -> Json<DashboardInfo> {
    trace!("Request received");
    
    let log_array = log_array.read().await;
    let total_count = log_array.len() as u32;
    let mut error_count = 0;
    let mut warning_count = 0;

    for entry in log_array.iter() {
        match entry.level {
            crate::LogLevel::Error => error_count += 1,
            crate::LogLevel::Warn => warning_count += 1,
            _ => {}
        }
    }

    Json(DashboardInfo {
        total_count,
        error_count,
        warning_count,
    })
}