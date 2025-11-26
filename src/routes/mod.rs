pub mod components;
pub mod staticsystem;
pub mod system;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(staticsystem::routes())
        .merge(system::routes())
        .merge(components::routes())
}
