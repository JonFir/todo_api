use std::{error::Error, sync::Arc};

use common::configuration::{AppState, Environment};

mod common;
mod db;
mod features;
mod server;

pub async fn run() -> Result<(), Box<dyn Error>> {
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
