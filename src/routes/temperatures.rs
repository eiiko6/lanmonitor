use axum::{Json, Router, routing::get};
use serde::Serialize;
use sysinfo::Components;

#[derive(Serialize, Debug)]
struct Temperature {
    id: String,
    label: String,
    temp: String,
    max: String,
    critical: String,
}

pub fn routes() -> Router {
    Router::new().route("/temperatures", get(get_temperatures))
}

async fn get_temperatures() -> Json<Vec<Temperature>> {
    let components = Components::new_with_refreshed_list();

    let res = Json(
        components
            .iter()
            .map(|comp| Temperature {
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
            .collect(),
    );

    // tracing::info!("sent components: {res:?}");

    res
}
