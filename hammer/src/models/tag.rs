use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Tag {
    pub(crate) name: String,
    pub(crate) description: String,
}
