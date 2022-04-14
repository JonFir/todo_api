use actix_web::http::header::HeaderMap;

use super::Claims;
use crate::common::errors::{Error, ErrorMeta};

pub fn encode(
    user_id: String,
    secret: &str,
    minutes: i64,
) -> Result<String, Error> {
    let time = chrono::Utc::now() + chrono::Duration::minutes(minutes);
    let claims = Claims {
        sub: user_id,
        exp: time.timestamp() as usize,
    };
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(Error::from_parent)
}

pub fn decode(
    token: &str,
    secret: &str,
) -> Result<jsonwebtoken::TokenData<Claims>, Error> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(Error::from_parent)
}

pub fn extract_from_headers(
    headers: &HeaderMap,
    jwt_secret: &str,
) -> Result<jsonwebtoken::TokenData<Claims>, Error> {
    let token_parts = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .split(" ")
        .collect::<Vec<&str>>();

    if token_parts.len() != 2
        || !token_parts.first().unwrap_or(&"").eq(&"Bearer")
    {
        return Err(Error::from(ErrorMeta::ACCESS_TOKEN_MISSING));
    }
    let token = token_parts[1];
    decode(token, jwt_secret)
}
