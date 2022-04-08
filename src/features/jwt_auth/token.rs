use super::Claims;

pub fn encode(user_id: String, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let time = chrono::Utc::now() + chrono::Duration::days(365);
    let claims = Claims {
        sub: user_id,
        exp: time.timestamp() as usize,
    };
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode(token: &str, secret: &str) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
}
