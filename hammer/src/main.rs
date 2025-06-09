pub(crate) mod backend;
pub(crate) mod config;
pub(crate) mod frontend;

use crate::backend::routes::get_backend_routes;
use crate::frontend::init_templates;
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

    // Initialize templates
    let templates = init_templates().expect("Failed to initialize templates");

    let app = Router::<()>::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .nest("/api", get_backend_routes())
        .merge(frontend::routes::get_frontend_routes(templates))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("TRACE: Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
