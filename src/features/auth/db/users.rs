use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, Pool, Postgres};

use crate::{
    common::errors::{Error, ErrorMeta},
    features::auth::entity::User,
};

pub async fn create(
    pool: &Pool<Postgres>,
    username: &str,
    hash: &str,
    email: &str,
    email_verified: bool,
) -> Result<PgQueryResult, Error> {
    let dt = DateTime::<Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(61, 0),
        chrono::Utc,
    );
    sqlx::query!(
        "
    INSERT INTO users (username, hash, email, email_verified, created, updated, is_delete) 
    VALUES ($1, $2, $3, $4, $5, $6, $7)",
        username, hash, email, email_verified, dt, dt, false,
    )
    .execute(pool)
    .await
    .map_err(Error::from_db_error(|code| match code.as_ref() {
        "23505" => Some(Error::from(ErrorMeta::USER_EXIST)),
        _ => None,
    }))
}

pub async fn find_by_username(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<Option<User>, Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username,)
        .fetch_optional(pool)
        .await
        .map_err(Error::from_parent)
}
