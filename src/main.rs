use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{
    sync::Arc,
};
mod api;

#[derive(Deserialize, Debug)]
struct Environment {
    url: String,
    port: u16,
    database_url: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: Pool<Postgres>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let environment = make_environment().unwrap();
    let pool = make_pool(&environment.database_url).await.unwrap();
    let state = AppState {
        database: pool,
    };
    run_server(environment.url, environment.port, state).await
}

fn make_environment() -> Result<Environment, envy::Error> {
    envy::from_env::<Environment>()
}

async fn make_pool(uri: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(uri).await
}

async fn run_server(url: String, port: u16, state: AppState) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/auth")
                    .service(api::auth::handlers::register)
                    .service(api::auth::handlers::login),
            )
            .service(hello)
    })
    .bind((url, port))?
    .run()
    .await
}
