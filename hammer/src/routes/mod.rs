use std::sync::Arc;

mod abilities;
mod indexed;
mod items;
mod tags;

use crate::indexing::Index;
use axum::{
    Router,
    routing::{delete, get},
};

pub(crate) fn get_backend_routes(
    abilities_index: Arc<Index>,
    items_index: Arc<Index>,
) -> Router<()> {
    Router::new()
        .route("/indexed", get(indexed::get))
        .with_state(abilities_index)
        .with_state(items_index)
        .route("/tags", get(tags::get))
        .route("/abilities", get(abilities::find_all))
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
