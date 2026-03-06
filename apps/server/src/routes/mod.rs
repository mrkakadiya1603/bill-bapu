mod health;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/v1", api_routes())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

fn api_routes() -> Router {
    Router::new().merge(health::routes())
}
