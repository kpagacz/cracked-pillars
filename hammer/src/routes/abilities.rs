use crate::db::{delete_abbreviated_ability_by_slug, update_abbreviated_ability_by_slug};
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;

use crate::models::AbbreviatedAbility;

#[axum::debug_handler]
pub(crate) async fn delete(Path(slug): Path<String>) -> StatusCode {
    match delete_abbreviated_ability_by_slug(&slug) {
        Ok(_) => {
            tracing::event!(tracing::Level::DEBUG, "Deleted abbreviated ability: {slug}");
            StatusCode::NO_CONTENT
        }
        Err(e) => {
            tracing::event!(
                tracing::Level::ERROR,
                "Failed to delete abbreviated ability: {e}"
            );
            StatusCode::BAD_REQUEST
        }
    }
}

#[axum::debug_handler]
pub(crate) async fn update(
    Path(slug): Path<String>,
    Json(ability): Json<AbbreviatedAbility>,
) -> StatusCode {
    match update_abbreviated_ability_by_slug(&slug, ability) {
        Ok(_) => {
            tracing::event!(tracing::Level::DEBUG, "Updated abbreviated ability: {slug}");
            StatusCode::NO_CONTENT
        }
        Err(e) => {
            tracing::event!(
                tracing::Level::ERROR,
                "Failed to update abbreviated ability: {e}"
            );
            StatusCode::BAD_REQUEST
        }
    }
}
