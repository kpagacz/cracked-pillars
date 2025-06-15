use std::sync::Arc;

mod filter_abilities;
mod get_abilities;

use crate::routes::filter_abilities::filter_abilities;
use crate::routes::get_abilities::get_abilities;

use crate::{
    db::{get_connection, synchronize_db},
    index_abilities::index_abilities,
    read_abilities::read_abilities,
};
use axum::{routing::get, Router};

pub(crate) fn get_backend_routes() -> Router<()> {
    let db_connection = get_connection().unwrap();
    synchronize_db(&db_connection).unwrap();

    let abilities = Arc::new(read_abilities().unwrap());
    let index = Arc::new(index_abilities(&abilities));

    Router::new()
        .route(
            "/get-abilities",
            get({
                let abilities = Arc::clone(&abilities);
                move || get_abilities(abilities)
            }),
        )
        .route(
            "/filter-abilities",
            get({
                let abilities = Arc::clone(&abilities);
                let index = Arc::clone(&index);
                move |params| filter_abilities(params, abilities, index)
            }),
        )
}
