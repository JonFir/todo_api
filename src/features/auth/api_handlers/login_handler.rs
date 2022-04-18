use std::sync::Arc;

use crate::common::error_response::{ErrorMeta, ErrorResponse};
use crate::common::ResponsePayload;
use crate::features::auth::db::users;
use crate::features::auth::errors::Error;
use crate::features::auth::{jwt, password_hash, random_string};
use crate::AppState;

use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};

#[post("/login")]
pub async fn login_handler(
    payload: web::Json<Payload>,
    data: web::Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let response =
        make_response(payload, data)
            .await
            .map_err(|e| ErrorResponse {
                meta: ErrorMeta::USER_NOT_FOUND,
                parent: e.into(),
            })?;
    Ok(ResponsePayload::succes("User did created", response))
}

async fn make_response(
    payload: web::Json<Payload>,
    data: web::Data<Arc<AppState>>,
) -> Result<Response, Error> {
    let user = users::find_by_username(&data.database, &payload.login)
        .await?
        .ok_or(Error::IncorectLogin)?;
    let is_password_correct =
        password_hash::verify(&user.hash, &payload.password)?;

    if !is_password_correct {
        return Err(Error::IncorectPassword);
    }

    let refresh_token = random_string::new(256);
    users::update_refresh_token(&data.database, &user.id, Some(&refresh_token))
        .await?;

    let access_token = jwt::token::encode(
        user.id.to_string(),
        &data.environment.jwt_secret,
        data.environment.jwt_duration,
    )?;
    let response = Response {
        access_token,
        refresh_token,
    };

    Ok(response)
}

#[derive(Deserialize)]
pub struct Payload {
    login: String,
    password: String,
}

#[derive(Serialize)]
struct Response {
    access_token: String,
    refresh_token: String,
}
