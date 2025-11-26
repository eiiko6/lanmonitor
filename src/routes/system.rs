use axum::{Json, Router, routing::get};
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
struct SystemInfo {
    uptime: u64,
    cpu_usage: f32,
    used_memory: u64,
    total_memory: u64,
}

pub fn routes() -> Router {
    Router::new().route("/system", get(get_system))
}

async fn get_system() -> Json<SystemInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    Json(SystemInfo {
        uptime: System::uptime(),
        cpu_usage: sys.global_cpu_usage(),
        used_memory: sys.used_memory(),
        total_memory: sys.total_memory(),
    })
}
