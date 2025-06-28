mod jwt;
mod login;
mod middleware;
mod routes;
mod user;
mod verify;

use ::hmac::Mac;
use ::jwt::VerifyWithKey;
pub(crate) use middleware::auth_required;
pub(crate) use routes::auth_routes;

fn verify_token(token: &str) -> Result<jwt::MyJWT, crate::error::Error> {
    let secret = crate::CONFIG.auth_secret.as_bytes();
    let key: hmac::Hmac<sha2::Sha384> = hmac::Hmac::new_from_slice(secret)
        .inspect_err(|err| tracing::debug!("Hmac algorithm creation failed with: {err:?}"))?;
    let my_jwt: jwt::MyJWT = token
        .verify_with_key(&key)
        .inspect_err(|err| tracing::debug!("Verification failed with error: {err:?}"))?;
    Ok(my_jwt)
}
