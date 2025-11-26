pub mod staticsystem;
pub mod system;
pub mod temperatures;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(staticsystem::routes())
        .merge(system::routes())
        .merge(temperatures::routes())
}
