use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, types::Uuid, Pool, Postgres};

use crate::features::auth::{entity::User, errors::Error};

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
    .await.map_err(|error| {
        let e = error.as_database_error().and_then(|e| e.code());
        match e {
            Some(e) if e.eq("23505") => Error::DbUserAlreadyExist(error),
            _ => Error::DbUserCreateFail(error),
        }
    })
}

pub async fn find_by_username(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<Option<User>, Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username,)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::DbUserNotFound(e))
}

pub async fn find_by_id(
    pool: &Pool<Postgres>,
    id: &Uuid,
) -> Result<Option<User>, Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id,)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::DbUserNotFound(e))
}

pub async fn update_refresh_token(
    pool: &Pool<Postgres>,
    id: &Uuid,
    refresh_token: Option<&str>,
) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        "UPDATE users SET refresh_token = $1 WHERE id = $2",
        refresh_token,
        id,
    )
    .execute(pool)
    .await
    .map_err(|e| Error::DbUpdateRefreshTokenFail(e))
}
