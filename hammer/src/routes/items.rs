use crate::db::{self, item};
use crate::error::Error;
use crate::models::{Item, JsonItem};
use axum::{
    Json,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[axum::debug_handler]
pub(super) async fn delete(Path(slug): Path<String>) -> Result<StatusCode, Error> {
    let conn = db::get_connection()?;
    item::delete(&slug, &conn)?;
    Ok(StatusCode::NO_CONTENT)
}

#[axum::debug_handler]
pub(super) async fn update(
    Path(slug): Path<String>,
    Json(item): Json<JsonItem>,
) -> Result<StatusCode, Error> {
    let mut conn = db::get_connection()?;
    item::update(&slug, &item.into(), &mut conn)?;
    Ok(StatusCode::NO_CONTENT)
}

#[axum::debug_handler]
pub(super) async fn find_all() -> Result<Json<Vec<Item>>, Error> {
    let conn = db::get_connection()?;
    Ok(Json(
        item::find_all(&conn)?.into_iter().map(Item::from).collect(),
    ))
}

#[axum::debug_handler]
pub(super) async fn find_by_slug(Path(slug): Path<String>) -> Result<Response, Error> {
    let conn = db::get_connection()?;
    if let Some(item) = item::find_by_slug(&slug, &conn)? {
        Ok((StatusCode::OK, Json(Item::from(item))).into_response())
    } else {
        Ok((StatusCode::NOT_FOUND).into_response())
    }
}

#[axum::debug_handler]
pub(super) async fn insert(Json(item): Json<JsonItem>) -> Result<StatusCode, Error> {
    let mut conn = db::get_connection()?;
    let item = Item::from(item);
    item::insert(&item, &mut conn)?;
    Ok(StatusCode::CREATED)
}
