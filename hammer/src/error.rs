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
