use axum::{Json, Router, routing::get};
use serde::Serialize;
use sysinfo::Components;

#[derive(Serialize)]
struct Temperature {
    label: String,
    temp: String,
}

pub fn routes() -> Router {
    Router::new().route("/temps", get(get_temps))
}

async fn get_temps() -> Json<Vec<Temperature>> {
    let components = Components::new_with_refreshed_list();

    let temps = components
        .iter()
        .map(|comp| Temperature {
            label: comp.label().to_string(),
            temp: comp
                .temperature()
                .map_or(String::from("unknown"), |temp| temp.to_string()),
        })
        .collect();

    Json(temps)
}
