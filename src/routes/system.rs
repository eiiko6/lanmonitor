use axum::{Json, Router, routing::get};
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
struct SystemInfo {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
}

pub fn routes() -> Router {
    Router::new().route("/system", get(get_system))
}

async fn get_system() -> Json<SystemInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    Json(SystemInfo {
        cpu_usage: sys.global_cpu_usage(),
        memory_used: sys.used_memory(),
        memory_total: sys.total_memory(),
    })
}
