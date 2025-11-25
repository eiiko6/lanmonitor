pub mod sensors;
pub mod system;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(system::routes())
        .merge(sensors::routes())
}
