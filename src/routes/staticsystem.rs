use std::fs;
use std::time::SystemTime;

use axum::{Json, Router, routing::get};
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize, Debug)]
struct StaticSystemInfo {
    os_version: String,
    kernel_version: String,
    distribution_id: String,

    // user: String,
    // local_ip: String,
    os_age: String,

    core_count: String,
    total_memory: u64,
}

pub fn routes() -> Router {
    Router::new().route("/static-system", get(get_static_system))
}

// Root filesystem age in days
fn os_age_days() -> Option<u64> {
    let root_metadata = fs::metadata("/").ok()?;
    let creation_time = root_metadata.created().ok()?; // birth time
    let now = SystemTime::now();
    let duration = now.duration_since(creation_time).ok()?;
    Some(duration.as_secs() / 86400) // seconds to days
}

async fn get_static_system() -> Json<StaticSystemInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let res = Json(StaticSystemInfo {
        os_version: System::long_os_version().unwrap_or(String::from("unknown")),
        kernel_version: System::kernel_long_version(),
        distribution_id: System::distribution_id(),

        os_age: os_age_days()
            .map(|days| days.to_string())
            .unwrap_or_else(|| "unknown".to_string()),

        core_count: System::physical_core_count()
            .map_or(String::from("unknown"), |count| count.to_string()),
        total_memory: sys.total_memory(),
    });

    // tracing::info!("sent system: {res:?}");

    res
}
