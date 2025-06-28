use crate::{auth::jwt::MyJWT, error::Error};
use axum::extract::{Json, Query};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct VerifyParams {
    auth_token: String,
}

#[axum::debug_handler]
#[tracing::instrument(level = "trace")]
pub(super) async fn verify(Query(params): Query<VerifyParams>) -> Result<Json<MyJWT>, Error> {
    Ok(Json(super::verify_token(&params.auth_token).inspect_err(
        |_| tracing::error!("No auth_token parameter"),
    )?))
}
