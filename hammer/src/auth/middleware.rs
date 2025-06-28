use crate::{
    auth::user::Role,
    error::{Error, ErrorType},
};

use axum::{
    extract::Request,
    http::{HeaderMap, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

pub(crate) async fn auth_required(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, Error> {
    match get_token(&headers) {
        Some(token) if is_token_valid(token)? => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(Error(
            String::from("The token is invalid"),
            ErrorType::Forbidden,
        )),
    }
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers.get(AUTHORIZATION).and_then(|auth_value| {
        let value: &str = auth_value.to_str().unwrap_or("");
        let (_, token) = value.split_once(' ')?;

        Some(token)
    })
}

fn is_token_valid(token: &str) -> Result<bool, Error> {
    let jwt = super::verify_token(token)?;
    Ok(matches!(jwt.role, Role::Admin | Role::Editor))
}
