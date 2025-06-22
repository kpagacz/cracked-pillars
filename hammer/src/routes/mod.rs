use std::sync::Arc;

mod abilities;
mod items;
mod tags;

use crate::{index_abilities::index_abilities, read_abilities::read_abilities};
use axum::{
    Router,
    routing::{delete, get, patch},
};

pub(crate) fn get_backend_routes() -> Router<()> {
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
        .route(
            "/abilities/{slug}",
            delete(abilities::delete).patch(abilities::update),
        )
        .route("/items", get(items::find_all).post(items::insert))
        .route(
            "/items/{slug}",
            get(items::find_by_slug)
                .patch(items::update)
                .delete(items::delete),
        )
}
