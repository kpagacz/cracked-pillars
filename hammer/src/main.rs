pub(crate) mod db;
pub(crate) mod error;
pub(crate) mod import_from_quarry;
pub(crate) mod indexing;
pub(crate) mod middleware;
pub(crate) mod models;
pub(crate) mod routes;

use crate::{
    import_from_quarry::import_to_db,
    indexing::{index_abilities, index_items},
    routes::get_backend_routes,
};
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

    // Import files to database if needed
    let args: Vec<_> = std::env::args().collect();
    if let Some("--import-from-quarry") = args.get(1).map(String::as_str) {
        import_from_quarry();
    }

    // Index abilities and items
    let abilities_index = index_abilities(&conn).expect("Indexing succeeds");
    let items_index = index_items(&conn).expect("Indexing succeeds");

    let app = Router::<()>::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .nest("/api", get_backend_routes(abilities_index, items_index))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::debug!("TRACE: Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn import_from_quarry() {
    match import_to_db() {
        Ok(_) => {
            tracing::info!("Successfully imported abilities and items from JSON files to database")
        }
        Err(e) => panic!("Failed to import abilities and items to database: {e:?}"),
    }
}
