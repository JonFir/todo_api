use configuration::{AppState, Environment};
use log::error;

mod api;
mod db;
mod middleware;
mod server;
use std::{sync::Arc, error::Error};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let result = run().await;
    if let Err(e) = result.as_ref() {
        error!("Critical error: {:?}", e)
    }
    result
}

async fn run() -> Result<(), Box<dyn Error>> {
    let environment = Environment::load()?;
    let address = environment.socket_addrs();
    let pool = db::make_pool(&environment.database_url).await?;
    let state = AppState {
        database: pool,
        environment,
    };
    let state = Arc::new(state);
    server::run(address, state).await?;
    Ok(())
}
