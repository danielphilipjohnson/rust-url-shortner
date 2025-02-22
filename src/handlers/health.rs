use axum::{response::IntoResponse, routing::get, Json, Router};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::thread;
use std::{
    sync::atomic::{AtomicI32, AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static START_TIME: Lazy<u64> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
});

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);
static CPU_USAGE_SIMULATOR: AtomicI32 = AtomicI32::new(0);

#[derive(Serialize)]
pub struct MetricsResponse {
    uptime_seconds: u64,
    memory_usage_mb: u64,
    cpu_usage_percent: f32,
    total_requests: u64,
    requests_per_minute: f32,
}

pub fn start_cpu_usage_simulator() {
    thread::spawn(|| {
        loop {
            let usage = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                % 60) as i32;
            CPU_USAGE_SIMULATOR.store(usage, Ordering::Relaxed);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

pub async fn metrics_handler() -> impl IntoResponse {
    let total_requests = REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let uptime = current_time - *START_TIME;

    let requests_per_minute = if uptime > 0 {
        (total_requests as f32 / uptime as f32) * 60.0
    } else {
        0.0
    };

    let metrics = MetricsResponse {
        uptime_seconds: uptime,
        memory_usage_mb: get_memory_usage(),
        cpu_usage_percent: get_cpu_usage(),
        total_requests,
        requests_per_minute,
    };

    Json(metrics)
}

fn get_memory_usage() -> u64 {
    let base_memory = 50;
    let requests = REQUEST_COUNTER.load(Ordering::Relaxed);

    let request_memory = (requests / 1000) * 5; // Add 5MB per 1000 requests
    base_memory + request_memory
}

fn get_cpu_usage() -> f32 {
    let cpu_usage = CPU_USAGE_SIMULATOR.load(Ordering::Relaxed);
    (cpu_usage as f32 / 60.0) * 100.0
}

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}

pub fn health_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics_handler))
}

pub fn init_metrics() {
    start_cpu_usage_simulator();
}
