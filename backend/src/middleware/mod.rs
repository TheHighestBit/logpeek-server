use std::sync::Arc;
use std::time::{Duration, SystemTime};
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use log::{error, info};
use ringbuffer::RingBuffer;
use tokio::sync::Mutex;
use crate::{SETTINGS, SharedState};
use crate::log_reader::load_logs;

pub async fn authentication_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(header::AUTHORIZATION).and_then(|val| val.to_str().ok());

    if let Some(auth_header) = auth_header {
        let trimmed_header = auth_header.trim();
        let parts: Vec<&str> = trimmed_header.split_whitespace().collect();

        if parts.len() == 2 && parts[0] == "Basic" {
            let decoded_credentials = match BASE64_STANDARD.decode(parts[1]) {
                Ok(bytes) => bytes,
                Err(e) => {
                    error!("Base64 Decode Error: {}", e);
                    return Err(StatusCode::BAD_REQUEST);
                }
            };
            let decoded_string = String::from_utf8(decoded_credentials).map_err(|_| StatusCode::BAD_REQUEST)?;
            let auth_parts: Vec<&str> = decoded_string.splitn(2, ':').collect();

            if SETTINGS.read().await.get_string("main.secret").expect("Failed to read main.secret") == auth_parts[1] {
                Ok(next.run(req).await)
            } else {
                error!("Invalid credentials");
                Err(StatusCode::UNAUTHORIZED)
            }
        } else {
            error!("Invalid authorization header format");
            Err(StatusCode::BAD_REQUEST)
        }
    } else {
        error!("Missing authorization header");
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn buffer_refresh_middleware(State(shared_state): State<SharedState>, req: Request, next: Next) -> Result<Response, StatusCode> {
    let mut last_buffer_update = shared_state.last_buffer_update.lock().await;
    let update_cooldown = Duration::from_secs(SETTINGS.read().await.get_int("main.buffer_update_cooldown").unwrap_or(10) as u64);

    if last_buffer_update.elapsed().unwrap_or(Duration::from_secs(15)) > update_cooldown || req.headers().contains_key("force-refresh") {
        load_logs(shared_state.log_buffer.clone(), shared_state.cache.clone()).await;
        *last_buffer_update = SystemTime::now();

        info!("Log entries updated");
    }

    Ok(next.run(req).await)
}