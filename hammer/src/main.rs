pub(crate) mod db;
pub(crate) mod error;

use axum::{Router, routing::get};
use db::{get_connection, synchronize_db};

#[tokio::main]
async fn main() {
    let db_connection = get_connection().unwrap();
    synchronize_db(&db_connection).unwrap();

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    const ADDRESS: &str = "0.0.0.0:3000";
    println!("Serving on {ADDRESS}");
    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
