use axum::{Router, routing::get};

pub(crate) fn frontend_routes() -> Router {
    Router::new().route("/", get(|| async { "Welcome to the Hammer Frontend!" }))
}
