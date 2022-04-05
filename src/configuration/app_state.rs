use sqlx::{Pool, Postgres};

use crate::environment::Environment;

#[derive(Clone)]
pub struct AppState {
    pub database: Pool<Postgres>,
    pub environment: Environment,
}
