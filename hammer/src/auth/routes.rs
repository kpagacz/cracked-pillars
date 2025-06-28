use super::{login::login, verify::verify};
use axum::{
    Router,
    routing::{get, post},
};

pub(crate) fn auth_routes() -> Router<()> {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/verify", get(verify))
            .route("/login", post(login)),
    )
}
