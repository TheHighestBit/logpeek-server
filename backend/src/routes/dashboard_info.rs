use std::cmp::min;
use std::collections::HashMap;
use std::ops::Add;

use axum::extract::{Query, State};
use axum::Json;
use log::{debug, trace};
use ringbuffer::RingBuffer;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime, Time};

use crate::{convert_app_to_i, LogBufferIterator, SharedState};

#[derive(Debug, Deserialize)]
pub struct Params {
    application: Option<String>,
    utc_offset: i64, // in minutes
}

#[derive(Serialize)]
pub struct DashboardResponse {
    total_logs_24: [u32; 24],
    error_logs_24: [u32; 24],
    warning_logs_24: [u32; 24],
    total_logs_week: [u32; 7],
    error_logs_week: [u32; 7],
    warning_logs_week: [u32; 7],
    top_modules_24: Vec<(String, f32)>,
    top_modules_week: Vec<(String, f32)>,
    log_buffer_usage: f32,
    total_log_entries: u32,
}

pub async fn dashboard_info_handler(
    Query(params): Query<Params>,
    State(shared_state): State<SharedState>,
) -> Json<DashboardResponse> {
    trace!("Request received {:?}", params);

    let application = if let Some(param_application) = &params.application {
        convert_app_to_i(param_application, &shared_state.i_to_app.lock().await)
    } else {
        None
    };

    debug!("Application filter after conversion: {:?}", application);

    let current_time = OffsetDateTime::now_utc();
    let start_of_tomorrow = current_time
        .replace_time(Time::MIDNIGHT)
        .add(Duration::days(1));

    let mut total_logs_24: [u32; 24] = [0; 24];
    let mut error_logs_24: [u32; 24] = [0; 24];
    let mut warning_logs_24: [u32; 24] = [0; 24];
    let mut total_logs_week: [u32; 7] = [0; 7];
    let mut error_logs_week: [u32; 7] = [0; 7];
    let mut warning_logs_week: [u32; 7] = [0; 7];
    let mut module_counter_tree: HashMap<String, u32> = HashMap::new();
    let mut top_modules_24: Vec<(String, f32)> = Vec::new();
    let mut flag_24 = false;

    let log_buffer_map = shared_state.log_buffer.read().await;
    let buffer_iterator = LogBufferIterator::new(&log_buffer_map, application);

    for entry in
        buffer_iterator.take_while(|entry| entry.timestamp >= start_of_tomorrow - Duration::days(7))
    {
        if entry.timestamp > current_time - Duration::hours(24) {
            let hour: usize = (current_time - entry.timestamp).whole_hours() as usize;
            match entry.level {
                log::Level::Error => error_logs_24[hour] += 1,
                log::Level::Warn => warning_logs_24[hour] += 1,
                _ => {}
            }

            total_logs_24[hour] += 1;
        } else if !flag_24 {
            let mut module_count: Vec<(String, u32)> = module_counter_tree
                .iter()
                .map(|(module, count)| (module.clone(), *count))
                .collect();

            module_count.sort_by(|a, b| b.1.cmp(&a.1));

            let total_24_errors = error_logs_24.iter().sum::<u32>();
            top_modules_24 = module_count
                .iter()
                .take(5)
                .map(|(module, count)| {
                    (
                        module.clone(),
                        *count as f32 * 100.0 / total_24_errors as f32,
                    )
                })
                .collect();

            flag_24 = true;
        }

        let day: usize = min(
            (start_of_tomorrow - entry.timestamp + Duration::minutes(params.utc_offset))
                .whole_days() as usize,
            6,
        );
        match entry.level {
            log::Level::Error => {
                error_logs_week[day] += 1;
                *module_counter_tree.entry(entry.module.clone()).or_insert(0) += 1;
            }
            log::Level::Warn => warning_logs_week[day] += 1,
            _ => {}
        }

        total_logs_week[day] += 1;
    }

    let mut module_count: Vec<(String, u32)> = module_counter_tree
        .iter()
        .map(|(module, count)| (module.clone(), *count))
        .collect();

    module_count.sort_by(|a, b| b.1.cmp(&a.1));

    let total_week_errors = error_logs_week.iter().sum::<u32>();
    let top_modules_week: Vec<(String, f32)> = module_count
        .into_iter()
        .take(5)
        .map(|(module, count)| (module, count as f32 * 100.0 / total_week_errors as f32))
        .collect();

    // Special case when all logs are from past 24h
    if !flag_24 {
        top_modules_24 = top_modules_week.clone();
    }

    let mut total_length = 0;
    let mut total_capacity = 0;

    log_buffer_map
        .iter()
        .filter(|entry| application.is_none() || application.unwrap() == *entry.0)
        .for_each(|entry| {
            total_length += entry.1.len();
            total_capacity += entry.1.capacity();
        });

    Json(DashboardResponse {
        total_logs_24,
        error_logs_24,
        warning_logs_24,
        total_logs_week,
        error_logs_week,
        warning_logs_week,
        top_modules_24,
        top_modules_week,
        log_buffer_usage: total_length as f32 / total_capacity as f32 * 100.0,
        total_log_entries: total_length as u32,
    })
}
