use axum::extract::Json;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        jwt::MyJWT,
        user::{Role, User},
    },
    db,
    error::Error,
};

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct UserInfo {
    sub: String,
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
    email: String,
    email_verified: bool,
}

#[derive(Debug, Serialize)]
pub(super) struct LoginResponse {
    jwt: String,
    email: String,
    role: Role,
}

#[axum::debug_handler]
#[tracing::instrument(level = "trace")]
pub(super) async fn login(Json(user_info): Json<UserInfo>) -> Result<Json<LoginResponse>, Error> {
    let conn = db::get_connection()?;
    let mut stmt = conn.prepare_cached("SELECT email, role FROM users WHERE email=?1")?;
    let mut rows = stmt.query(rusqlite::params![&user_info.email])?;
    let user: User;
    if let Some(row) = rows.next()? {
        tracing::trace!("User found in the database: {:?}", row);
        let email: String = row.get(0)?;
        let role: Role = row.get(1)?;
        user = User { email, role }
    } else {
        user = User {
            email: user_info.email,
            role: Role::Viewer,
        }
    }
    let jwt = MyJWT {
        email: user.email,
        role: user.role,
    };
    tracing::trace!("Creating JWT for user: {:?}", jwt);
    let token = jwt.signed_token()?;
    Ok(Json(LoginResponse {
        jwt: token,
        email: jwt.email,
        role: jwt.role,
    }))
}
