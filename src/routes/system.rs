use axum::{Json, Router, routing::get};
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize, Debug)]
struct SystemInfo {
    uptime: u64,
    cpu_usage: f32,
    used_memory: u64,
    total_memory: u64,
    gpu_usage: String,
    gpu_power_usage: String,
    gpu_power_cap: String,
}

pub fn routes() -> Router {
    Router::new().route("/system", get(get_system))
}

fn read_amd_gpu_load() -> Option<u32> {
    // Usually the GPU is card1, but iterate for robustness.
    for idx in 0..10 {
        let path = format!("/sys/class/drm/card{idx}/device/gpu_busy_percent");
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(v) = content.trim().parse::<u32>() {
                return Some(v);
            }
        }
    }
    None
}

fn read_amd_gpu_power_watts() -> Option<(f32, f32)> {
    let hwmon_base = format!("/sys/class/drm/card1/device/hwmon");
    let hwmon_dirs = std::fs::read_dir(&hwmon_base).ok()?;

    for entry in hwmon_dirs.flatten() {
        let avg_path = entry.path().join("power1_average");
        let cap_path = entry.path().join("power1_cap_max");

        let microwatts = std::fs::read_to_string(&avg_path)
            .ok()?
            .trim()
            .parse::<u64>()
            .ok()?;
        let cap = std::fs::read_to_string(&cap_path)
            .ok()?
            .trim()
            .parse::<u64>()
            .ok()?;

        return Some((microwatts as f32 / 1_000_000.0, cap as f32 / 1_000_000.0));
    }

    None
}

async fn get_system() -> Json<SystemInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let gpu_power = read_amd_gpu_power_watts();

    let res = Json(SystemInfo {
        uptime: System::uptime(),

        cpu_usage: sys.global_cpu_usage(),

        used_memory: sys.used_memory(),
        total_memory: sys.total_memory(),

        gpu_usage: read_amd_gpu_load().map_or(String::from("unknown"), |val| val.to_string()),

        gpu_power_usage: gpu_power
            .as_ref()
            .map_or(String::from("unknown"), |v| v.0.to_string()),
        gpu_power_cap: gpu_power
            .as_ref()
            .map_or(String::from("unknown"), |v| v.1.to_string()),
    });

    // tracing::info!("sent system: {res:?}");

    res
}
