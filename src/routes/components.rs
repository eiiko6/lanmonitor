use axum::{Json, Router, routing::get};
use serde::Serialize;
use sysinfo::Components;

#[derive(Serialize)]
struct Component {
    id: String,
    label: String,
    temp: String,
    max: String,
    critical: String,
}

pub fn routes() -> Router {
    Router::new().route("/components", get(get_components))
}

async fn get_components() -> Json<Vec<Component>> {
    let components = Components::new_with_refreshed_list();

    let temps = components
        .iter()
        .map(|comp| Component {
            id: comp.id().map_or(String::default(), |temp| temp.to_string()),
            label: comp.label().to_string(),
            temp: comp
                .temperature()
                .map_or(String::from("unknown"), |temp| temp.to_string()),
            max: comp
                .max()
                .map_or(String::from("unknown"), |temp| temp.to_string()),
            critical: comp
                .critical()
                .map_or(String::from("unknown"), |temp| temp.to_string()),
        })
        .collect();

    Json(temps)
}
