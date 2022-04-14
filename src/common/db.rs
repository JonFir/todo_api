use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn make_pool(
    uri: &str,
    max_connections: u32,
) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(uri)
        .await
}
