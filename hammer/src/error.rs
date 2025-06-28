use std::fmt::Display;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response,
};

#[derive(Debug)]
pub(crate) struct Error(pub(crate) String, pub(crate) ErrorType);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Cracked Pillars Error. Type: {:?} | Message: {}",
            self.1, self.0
        )
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub(crate) enum ErrorType {
    Runtime,
    Forbidden,
    Cryptography,
    NotFound,
}

impl Error {
    pub(crate) fn new(text: String) -> Self {
        Self(text, ErrorType::Runtime)
    }
}

impl From<hmac::digest::InvalidLength> for Error {
    fn from(_: hmac::digest::InvalidLength) -> Self {
        Self(
            String::from("Key has invalid lenth"),
            ErrorType::Cryptography,
        )
    }
}

impl From<jwt::Error> for Error {
    fn from(_: jwt::Error) -> Self {
        Self(
            String::from("Error signing the token"),
            ErrorType::Cryptography,
        )
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::new(value.to_string())
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Self::new(value.to_string())
    }
}

#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    message: String,
}

impl response::IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let status_code = match self.1 {
            ErrorType::Runtime | ErrorType::Cryptography => StatusCode::BAD_REQUEST,
            ErrorType::Forbidden => StatusCode::FORBIDDEN,
            ErrorType::NotFound => StatusCode::NOT_FOUND,
        };
        Response::builder()
            .status(status_code)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!(ErrorResponse { message: self.0 }).to_string(),
            ))
            .unwrap_or_else(|_| Response::new(axum::body::Body::empty()))
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::new(format!("IO Error: {value:?}"))
    }
}
