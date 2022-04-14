use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hash: String,
    pub refresh_token: Option<String>,
    pub email: String,
    pub email_verified: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub is_delete: bool,
}
