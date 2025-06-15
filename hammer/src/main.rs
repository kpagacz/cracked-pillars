pub(crate) mod ability;
pub(crate) mod config;
pub(crate) mod db;
pub(crate) mod error;
pub(crate) mod index_abilities;
pub(crate) mod load_abilities;
pub(crate) mod read_abilities;
pub(crate) mod routes;

use crate::routes::get_backend_routes;
use axum::{Router, http::StatusCode, routing::get};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tower_http=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::<()>::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .nest("/api", get_backend_routes())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("TRACE: Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
