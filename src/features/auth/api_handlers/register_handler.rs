use std::sync::Arc;

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Result,
};
use serde::Deserialize;

use crate::{
    common::error::Error,
    configuration::AppState,
    features::auth::{db, password_hash},
};

#[post("/register")]
pub async fn register(
    payload: Json<RegisterPayload>,
    data: Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let hash = password_hash::new(&payload.password)?;
    db::users::create(
        &data.database,
        &payload.login,
        &hash,
        &payload.email,
        false,
    )
    .await?;
    Ok(HttpResponse::Created().finish())
}

#[derive(Deserialize, Debug)]
pub struct RegisterPayload {
    login: String,
    password: String,
    email: String,
}
