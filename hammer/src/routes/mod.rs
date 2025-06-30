mod abilities;
mod indexed;
mod items;
mod tags;

use crate::auth::auth_required;
use axum::{
    Router,
    handler::Handler,
    routing::{delete, get, patch},
};

pub(crate) fn get_backend_routes() -> Router<()> {
    Router::new()
        .route("/indexed", get(indexed::get))
        .route("/tags", get(tags::get))
        .route("/abilities", get(abilities::find_all))
        .route(
            "/abilities/{slug}",
            delete(abilities::delete.layer(axum::middleware::from_fn(auth_required)))
                .patch(abilities::update.layer(axum::middleware::from_fn(auth_required))),
        )
        .route("/abilities/{slug}", get(abilities::find_by_slug))
        .route(
            "/abilities/{slug}/tags",
            patch(abilities::update_tags.layer(axum::middleware::from_fn(auth_required))),
        )
        .route(
            "/items",
            get(items::find_all)
                .post(items::insert.layer(axum::middleware::from_fn(auth_required))),
        )
        .route(
            "/items/{slug}",
            get(items::find_by_slug)
                .patch(items::update.layer(axum::middleware::from_fn(auth_required)))
                .delete(items::delete.layer(axum::middleware::from_fn(auth_required))),
        )
        .route(
            "/items/{slug}/tags",
            patch(items::update_tags.layer(axum::middleware::from_fn(auth_required))),
        )
}
