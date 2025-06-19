use std::sync::Arc;

pub(crate) mod abilities;
pub(crate) mod tags;

use crate::{
    db::{get_connection, synchronize_db},
    index_abilities::index_abilities,
    read_abilities::read_abilities,
};
use axum::{Router, routing::get};

pub(crate) fn get_backend_routes() -> Router<()> {
    let db_connection = get_connection().unwrap();
    synchronize_db(&db_connection).unwrap();

    let abilities = Arc::new(read_abilities().unwrap());
    let index = Arc::new(index_abilities(&abilities));

    Router::new()
        .route(
            "/abilities",
            get({
                let abilities = Arc::clone(&abilities);
                let index = Arc::clone(&index);
                move |params| abilities::get(params, abilities, index)
            }),
        )
        .route(
            "/tags",
            get({
                let index = Arc::clone(&index);
                move || tags::get(index)
            }),
        )
}
