use std::borrow::BorrowMut;

use crate::AppState;

use super::payloads::RegisterPayload;
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlx::types::chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::types::{chrono, Uuid};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(sqlx::FromRow)]
struct User {
    id: Uuid,
    username: String,
    hash: String,
    email: String,
    email_verified: bool,
    created: chrono::DateTime<chrono::Utc>,
    updated: chrono::DateTime<chrono::Utc>,
    is_delete: bool,
}

#[post("/register")]
pub async fn register(
    payload: web::Json<RegisterPayload>,
    data: web::Data<AppState>,
) -> impl Responder {
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    let config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 15360,
        time_cost: 10,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 128,
    };
    let hash = argon2::hash_encoded(payload.password.as_bytes(), salt.as_bytes(), &config).unwrap();

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.login)
        .fetch_optional(&data.database)
        .await
        .unwrap();

    if user.is_none() {
        HttpResponse::Ok().body("user exist");
    }
    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
    sqlx::query("
    INSERT INTO users (username, password, email, email_verified, created, updated, is_delete) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
    .bind(&payload.login)
    .bind(&hash)
    .bind(&payload.email)
    .bind(false)
    .bind(dt)
    .bind(dt)
    .bind(false)
    .execute(&data.database)
    .await
    .unwrap();
    HttpResponse::Ok().body("ok")
}

#[post("/login")]
pub async fn login(
    payload: web::Json<RegisterPayload>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.login)
        .fetch_optional(&data.database)
        .await
        .unwrap();
    if user.is_none() {
        return HttpResponse::Ok().body("user not found");
    }
    let user = user.unwrap();
    let result = argon2::verify_encoded(&user.hash, payload.password.as_bytes()).unwrap();
    if result {
        return HttpResponse::Ok().body("login");
    } else {
        return HttpResponse::Ok().body("usser not found 2");
    }
    
}
