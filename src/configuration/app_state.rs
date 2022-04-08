use sqlx::{Pool, Postgres};

use super::Environment;

#[derive(Clone)]
pub struct AppState {
    pub database: Pool<Postgres>,
    pub environment: Environment,
}
