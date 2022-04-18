use actix_web::http::header::HeaderMap;

use crate::features::auth::errors::Error;

use super::Claims;

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
    .map_err(|e| Error::InvalidTokenData(e))
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
    .map_err(|e| Error::InvalidToken(e))
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
        return Err(Error::MissingToken);
    }
    let token = token_parts[1];
    decode(token, jwt_secret)
}
