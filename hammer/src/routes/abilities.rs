use crate::db;
use crate::error::{Error, ErrorType};
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;

use crate::models::AbbreviatedAbility;

#[axum::debug_handler]
pub(super) async fn delete(Path(slug): Path<String>) -> StatusCode {
    match db::ability::delete_abbreviated_ability_by_slug(&slug) {
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
pub(super) async fn update(
    Path(slug): Path<String>,
    Json(ability): Json<AbbreviatedAbility>,
) -> StatusCode {
    match db::ability::update_abbreviated_ability_by_slug(&slug, ability) {
        Ok(_) => {
            tracing::event!(tracing::Level::DEBUG, "Updated abbreviated ability: {slug}");
            StatusCode::NO_CONTENT
        }
        Err(e) => {
            tracing::event!(
                tracing::Level::ERROR,
                "Failed to update abbreviated ability: {e:?}"
            );
            StatusCode::BAD_REQUEST
        }
    }
}

#[axum::debug_handler]
pub(super) async fn find_all() -> Result<Json<Vec<AbbreviatedAbility>>, Error> {
    let conn = db::get_connection()?;
    let abilities = db::ability::find_all(&conn)?
        .into_iter()
        .map(AbbreviatedAbility::from)
        .collect();
    Ok(Json(abilities))
}

#[axum::debug_handler]
#[tracing::instrument(level = "trace")]
pub(super) async fn update_tags(
    Path(slug): Path<String>,
    Json(new_tags): Json<Vec<String>>,
) -> Result<Json<Vec<String>>, Error> {
    let mut conn = db::get_connection()?;
    let new_tags =
        db::ability::update_tags_by_slug(&slug, new_tags, &mut conn).inspect_err(|err| {
            tracing::warn!("Error when updating the tags of the ability {slug}. Error: {err:?}")
        })?;
    Ok(Json(new_tags))
}

#[axum::debug_handler]
#[tracing::instrument(level = "trace")]
pub(super) async fn find_by_slug(
    Path(slug): Path<String>,
) -> Result<Json<AbbreviatedAbility>, Error> {
    let conn = db::get_connection()?;
    let ability = db::ability::find_by_slug(&slug, &conn)
        .inspect_err(|err| tracing::warn!("Error getting the ability with slug {slug}. {err:?}"))?;
    match ability {
        Some(ability) => Ok(Json(ability.into())),
        None => Err(Error("Ability not found".to_string(), ErrorType::NotFound)),
    }
}
