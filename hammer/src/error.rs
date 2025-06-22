use axum::{
    body::Body,
    http::{Response, StatusCode},
    response,
};

#[derive(Debug)]
pub(crate) struct Error(pub(crate) String);

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Self(value.to_string())
    }
}

impl response::IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(self.0))
            .unwrap_or_else(|_| Response::new(axum::body::Body::empty()))
    }
}
