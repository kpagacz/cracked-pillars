use crate::{CONFIG, auth::user::Role, error::Error};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha384;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct MyJWT {
    pub(super) email: String,
    pub(super) role: Role,
}

impl MyJWT {
    pub(super) fn signed_token(&self) -> Result<String, Error> {
        let secret = CONFIG.auth_secret.as_bytes();
        let key: Hmac<Sha384> = Hmac::new_from_slice(secret)?;
        Ok(self.sign_with_key(&key)?)
    }
}

impl TryFrom<String> for MyJWT {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let secret = CONFIG.auth_secret.as_bytes();
        let key: Hmac<Sha384> = Hmac::new_from_slice(secret)?;
        let token: MyJWT = value.verify_with_key(&key)?;
        Ok(token)
    }
}
