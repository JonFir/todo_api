use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::features::auth::errors::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn uuid(&self) -> Result<Uuid, Error> {
        Uuid::from_str(&self.sub).map_err(|_| Error::InvalidClaimsUuid)
    }
}
