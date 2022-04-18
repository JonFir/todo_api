use std::sync::Arc;

use actix_web::{post, web, HttpRequest, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        configuration::AppState,
        error_response::{ErrorMeta, ErrorResponse},
        ResponsePayload,
    },
    features::auth::{
        db::users,
        errors::Error,
        jwt::{self, token},
        random_string,
    },
};

#[post("/refresh_token")]
pub async fn refresh_token_handler(
    request: HttpRequest,
    payload: web::Json<Payload>,
    data: web::Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let response =
        make_response(request, payload, data).await.map_err(|e| {
            ErrorResponse {
                meta: ErrorMeta::USER_NOT_FOUND,
                parent: e.into(),
            }
        })?;
    Ok(ResponsePayload::succes("User did created", response))
}

async fn make_response(
    request: HttpRequest,
    payload: web::Json<Payload>,
    data: web::Data<Arc<AppState>>,
) -> Result<Response, Error> {
    let token = token::extract_from_headers(
        request.headers(),
        &data.environment.jwt_secret,
    )?;
    let id = token.claims.uuid()?;
    let user = users::find_by_id(&data.database, &id)
        .await?
        .ok_or(Error::UserNotFound)?;
    let refresh_token = user.refresh_token.unwrap_or("".into());
    if refresh_token.is_empty() || !refresh_token.eq(&payload.refresh_token) {
        return Err(Error::UserNotFound);
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
    pub refresh_token: String,
}

#[derive(Serialize)]
struct Response {
    access_token: String,
    refresh_token: String,
}
