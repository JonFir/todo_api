use std::sync::Arc;

use actix_web::{
    http::StatusCode,
    post,
    web::{Data, Json},
    Responder, Result,
};
use serde::Deserialize;

use crate::{
    common::{error::Error, ResponsePayload},
    configuration::AppState,
    features::auth::{db, password_hash},
};

#[post("/register")]
pub async fn register(payload: Json<RegisterPayload>, data: Data<Arc<AppState>>) -> Result<impl Responder, Error> {
    let hash = password_hash::new(&payload.password)?;
    db::users::create(&data.database, &payload.login, &hash, &payload.email, false).await?;
    let respose = ResponsePayload::succes_and_empty("User did registered".into())
        .customize()
        .with_status(StatusCode::CREATED);
    Ok(respose)
}

#[derive(Deserialize, Debug)]
pub struct RegisterPayload {
    login: String,
    password: String,
    email: String,
}
