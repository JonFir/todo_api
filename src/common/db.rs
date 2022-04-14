use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn make_pool(uri: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(uri).await
}