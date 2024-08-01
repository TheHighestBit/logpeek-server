use crate::SharedState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::trace;
use serde::Serialize;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, System};
use time::Duration;
use tokio::time::sleep;

#[derive(Serialize)]
pub struct SystemInfoResponse {
    memory_usage: String,
    total_memory: String,
    cpu_usage: String,
    os: String,
    host_name: String,
    uptime: String,
    server_uptime: String,
}

pub async fn sysinfo_handler(
    State(shared_state): State<SharedState>,
) -> (StatusCode, Json<SystemInfoResponse>) {
    trace!("Request received");

    let memory_usage;
    let cpu_usage;
    let total_memory;
    let uptime;
    // Using scopes here to drop the lock after the first refresh
    {
        let mut system = shared_state.sys.lock().await;
        system.refresh_cpu_usage();
    }

    sleep(std::time::Duration::from_millis(200)).await; // Need to wait for the CPU usage to be calculated correctly

    {
        let mut system = shared_state.sys.lock().await;

        system.refresh_specifics(
            sysinfo::RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::new().with_ram()),
        );

        memory_usage = format!(
            "{:.2}",
            system.used_memory() as f32 / (1024 * 1024 * 1024) as f32
        );
        total_memory = format!(
            "{:.2}",
            system.total_memory() as f32 / (1024 * 1024 * 1024) as f32
        );
        cpu_usage = format!("{:.2}", system.global_cpu_usage());
        uptime = Duration::seconds(System::uptime() as i64);
    }

    let server_uptime = shared_state
        .server_start_time
        .elapsed()
        .unwrap_or_else(|_| core::time::Duration::new(0, 0))
        .as_secs();

    (
        StatusCode::OK,
        Json(SystemInfoResponse {
            memory_usage,
            total_memory,
            cpu_usage,
            os: (*shared_state.os).clone(),
            host_name: (*shared_state.host_name).clone(),
            uptime: format!(
                "{}:{}:{}:{}",
                uptime.whole_days(),
                uptime.whole_hours() % 24,
                uptime.whole_minutes() % 60,
                uptime.whole_seconds() % 60
            ),
            server_uptime: format!(
                "{}:{}:{}:{}",
                server_uptime / 86400,
                (server_uptime % 86400) / 3600,
                (server_uptime % 3600) / 60,
                server_uptime % 60
            ),
        }),
    )
}
