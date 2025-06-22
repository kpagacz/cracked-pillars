pub(crate) mod db;
pub(crate) mod error;
pub(crate) mod import_from_quarry;
pub(crate) mod index_abilities;
pub(crate) mod middleware;
pub(crate) mod models;
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
            std::env::var("RUST_LOG").unwrap_or_else(|_| "hammer=trace,tower_http=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Synchronize database
    let conn = db::get_connection().expect("Failed to get DB connection");
    db::synchronize_db(&conn).expect("Failed to synchronize DB");

    let app = Router::<()>::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .nest("/api", get_backend_routes())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::debug!("TRACE: Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
