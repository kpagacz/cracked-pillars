use std::str::FromStr;

use crate::error::Error;

use rusqlite::types::{FromSql, FromSqlError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub(super) struct User {
    pub(super) email: String,
    pub(super) role: Role,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) enum Role {
    Editor,
    Admin,
    Viewer,
}

impl FromStr for Role {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "editor" => Role::Editor,
            "admin" => Role::Admin,
            _ => Role::Viewer,
        })
    }

    type Err = Error;
}

impl FromSql for Role {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_str()?
            .parse::<Role>()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}
