use crate::db::{self, tag};
use crate::error::Error;
use axum::Json;

#[axum::debug_handler]
pub(crate) async fn get() -> Result<Json<Vec<String>>, Error> {
    Ok(Json(tag::find_all(&db::get_connection()?)?))
}
