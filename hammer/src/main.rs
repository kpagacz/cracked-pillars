pub(crate) mod backend;
pub(crate) mod config;
pub(crate) mod frontend;

use crate::backend::routes::get_backend_routes;
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

    let app = Router::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .nest("/api", get_backend_routes())
        .nest("/", frontend::routes::frontend_routes())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    const ADDRESS: &str = "0.0.0.0:3000";
    println!("Serving on {ADDRESS}");
    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
