use std::sync::Arc;

use crate::common::error::ErrorMeta;
use crate::common::{error::Error, ResponsePayload};
use crate::features::auth::db::users;
use crate::features::auth::password_hash;
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
    if is_password_correct {
        let user_id = user.id.to_string();
        let secret = &data.environment.jwt_secret;
        let token = jwt_auth::token::encode(user_id, secret)?;
        let response = ResponsePayload {
            error: 0,
            message: String::from(""),
            data: LoginResponse { token },
        };
        Ok(response)
    } else {
        return Err(Error::from(ErrorMeta::USER_NOT_FOUND));
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    pub login: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

impl ResponsePayload<LoginResponse> {
    fn new(user_id: String, secret: &str) -> Result<Self, Error> {
        let token = jwt_auth::token::encode(user_id, secret)?;
        let response = ResponsePayload {
            error: 0,
            message: String::from(""),
            data: LoginResponse { token },
        };
        Ok(response)
    }
}
