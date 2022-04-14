use std::sync::Arc;

use crate::common::errors::ErrorMeta;
use crate::common::{errors::Error, ResponsePayload};
use crate::features::auth::db::users;
use crate::features::auth::{password_hash, random_string};
use crate::features::jwt_auth;
use crate::AppState;

use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};

#[post("/login")]
pub async fn login(
    payload: web::Json<LoginPayload>,
    data: web::Data<Arc<AppState>>,
) -> Result<impl Responder, Error> {
    let user = users::find_by_username(&data.database, &payload.login)
        .await?
        .ok_or(Error::from(ErrorMeta::USER_NOT_FOUND))?;
    let is_password_correct =
        password_hash::verify(&user.hash, &payload.password)?;

    if !is_password_correct {
        return Err(Error::from(ErrorMeta::USER_NOT_FOUND));
    }

    let refresh_token = random_string::new(256);
    users::update_refresh_token(&data.database, &user.id, Some(&refresh_token))
        .await?;

    let access_token = jwt_auth::token::encode(
        user.id.to_string(),
        &data.environment.jwt_secret,
        data.environment.jwt_duration,
    )?;
    let response = ResponsePayload::succes(
        "User did created".into(),
        LoginResponse {
            access_token,
            refresh_token,
        },
    );
    Ok(response)
}

#[derive(Deserialize)]
pub struct LoginPayload {
    login: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}
