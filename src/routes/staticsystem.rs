use axum::{Json, Router, routing::get};
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize, Debug)]
struct StaticSystemInfo {
    os_version: String,
    kernel_version: String,
    distribution_id: String,

    core_count: String,
    total_memory: u64,
}

pub fn routes() -> Router {
    Router::new().route("/static-system", get(get_static_system))
}

async fn get_static_system() -> Json<StaticSystemInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let res = Json(StaticSystemInfo {
        os_version: System::long_os_version().unwrap_or(String::from("unknown")),
        kernel_version: System::kernel_long_version(),
        distribution_id: System::distribution_id(),
        core_count: System::physical_core_count()
            .map_or(String::from("unknown"), |count| count.to_string()),
        total_memory: sys.total_memory(),
    });

    // tracing::info!("sent system: {res:?}");
    tracing::info!("sent static-system");

    res
}
