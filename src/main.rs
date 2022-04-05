use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use configuration::{AppState, Environment};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
mod api;
mod middleware;
use std::sync::Arc;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment = Environment::load().unwrap();
    let pool = make_pool(&environment.database_url).await.unwrap();
    let state = Arc::new(AppState {
        database: pool,
        environment: environment.clone(),
    });
    run_server(environment.url, environment.port, state).await
}

async fn make_pool(uri: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(uri).await
}

async fn run_server(url: String, port: u16, state: Arc<AppState>) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/auth")
                    .service(api::auth::handlers::register)
                    .service(api::auth::handlers::login),
            )
            .service(
                web::scope("/api")
                    .wrap(middleware::jwt_auth::Transformer {
                        app_state: state.clone(),
                    })
                    .service(hello),
            )
            .service(hello)
    })
    .bind((url, port))?
    .run()
    .await
}
