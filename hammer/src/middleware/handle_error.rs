use crate::error::Error;
use axum::http::StatusCode;

pub(crate) fn handle_error(err: Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.0.to_string())
}
