use std::collections::BTreeMap;

use axum::extract::State;
use axum::Json;
use axum_extra::extract::Query;
use log::{debug, trace};
use ringbuffer::RingBuffer;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{convert_app_to_i, LogEntry, SharedState};

#[derive(Debug, Deserialize)]
pub struct Params {
    applications: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct DashboardResponse {
    total_logs_24: [u32; 24],
    error_logs_24: [u32; 24],
    warning_logs_24: [u32; 24],
    total_logs_week: [u32; 7],
    error_logs_week: [u32; 7],
    warning_logs_week: [u32; 7],
    top_modules_24: Vec<(String, u32)>,
    top_modules_week: Vec<(String, u32)>,
    log_buffer_usage: f32,
    total_log_entries: u32,
}

pub async fn dashboard_info_handler(Query(params): Query<Params>, State(shared_state): State<SharedState>) -> Json<DashboardResponse> {
    trace!("Request received {:?}", params);

    let applications = if let Some(param_applications) = &params.applications {
        convert_app_to_i(param_applications, &shared_state.i_to_app.lock().await)
    } else {
        Vec::new()
    };

    debug!("Applications filter after conversion: {:?}", applications);

    let current_time = time::OffsetDateTime::now_utc();
    let mut total_logs_24: [u32; 24] = [0; 24];
    let mut error_logs_24: [u32; 24] = [0; 24];
    let mut warning_logs_24: [u32; 24] = [0; 24];
    let mut total_logs_week: [u32; 7] = [0; 7];
    let mut error_logs_week: [u32; 7] = [0; 7];
    let mut warning_logs_week: [u32; 7] = [0; 7];
    let mut module_counter_tree: BTreeMap<String, u32> = BTreeMap::new();
    let mut top_modules_24: Vec<(String, u32)> = Vec::new();
    let mut flag_24 = false;

    let log_buffer_map = shared_state.log_buffer.read().await;
    let mut iterators = log_buffer_map.iter().filter(|entry| applications.is_empty() || applications.contains(&entry.0))
        .map(|entry| entry.1.iter().rev().peekable())
        .collect::<Vec<_>>();
    
    loop {
        // First we need to find which buffer has the latest log entry
        let entry: &LogEntry;
        let mut latest_time: Option<&OffsetDateTime> = None;
        let mut index: usize = 0;
        
        for (i, iterator) in iterators.iter_mut().enumerate() {
            if let Some(peeked) = iterator.peek() {
                if let Some(current_latest) = latest_time {
                    if peeked.timestamp > *current_latest {
                        latest_time = Some(&peeked.timestamp);
                        index = i;
                    }
                } else {
                    latest_time = Some(&peeked.timestamp);
                    index = i;
                }
            }
        }
        
        if latest_time.is_some() {
            entry = iterators[index].next().unwrap(); // This unwrap is safe
        } else {
            break;
        }
        
        // No need to process logs older than 7 days
        if entry.timestamp < current_time - time::Duration::days(7) {
            break;
        }

        if entry.timestamp > current_time - time::Duration::hours(24) {
            let hour: usize = (current_time - entry.timestamp).whole_hours() as usize;
            match entry.level {
                log::Level::Error => error_logs_24[hour] += 1,
                log::Level::Warn => warning_logs_24[hour] += 1,
                _ => {}
            }

            total_logs_24[hour] += 1;
        } else if !flag_24 {
            // Only need the top 5 most common modules
            let mut module_count: Vec<(String, u32)> = module_counter_tree
                .iter()
                .map(|(module, count)| (module.clone(), *count))
                .collect();

            module_count.sort_by(|a, b| b.1.cmp(&a.1));

            let total_24_errors = error_logs_24.iter().sum::<u32>();
            top_modules_24 = module_count.into_iter().take(5)
                .map(|(module, count)| (module, count * 100 / total_24_errors))
                .collect();

            flag_24 = true;
        }

        let day: usize = (current_time - entry.timestamp).whole_days() as usize;
        match entry.level {
            log::Level::Error => {
                error_logs_week[day] += 1;
                *module_counter_tree.entry(entry.module.clone()).or_insert(0) += 1;
            },
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
    let top_modules_week: Vec<(String, u32)> = module_count.into_iter().take(5)
        .map(|(module, count)| (module, count * 100 / total_week_errors))
        .collect();
    
    // Special case when all logs are from past 24h
    if !flag_24 {
        top_modules_24 = top_modules_week.clone();
    }
    
    let mut total_length = 0;
    let mut total_capacity = 0;

    log_buffer_map.iter()
        .filter(|entry| applications.is_empty() || applications.contains(entry.0))
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