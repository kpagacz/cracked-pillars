use crate::{
    db::{self, tag},
    error::Error,
    models::Tag,
};
use axum::Json;

#[axum::debug_handler]
#[tracing::instrument(level = "trace")]
pub(crate) async fn get() -> Result<Json<Vec<Tag>>, Error> {
    Ok(Json(tag::find_all(&db::get_connection()?).inspect_err(
        |err| tracing::warn!("Failed to fetch all tags. {err:?}"),
    )?))
}
