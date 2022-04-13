use crate::common::errors::Error;

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
