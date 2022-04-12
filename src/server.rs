use std::sync::Arc;

use crate::configuration::AppState;
use crate::features::auth::api_handlers;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::features::jwt_auth;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn run(address: (String, u16), state: Arc<AppState>) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        let app_data = web::Data::new(Arc::clone(&state));

        let auth_scope = web::scope("/auth")
            .service(api_handlers::register)
            .service(api_handlers::login);

        let api_scope = web::scope("/api")
            .wrap(jwt_auth::Middleware {
                app_state: state.clone(),
            })
            .service(hello);

        let global_scope = hello;

        App::new()
            .app_data(app_data)
            .service(auth_scope)
            .service(api_scope)
            .service(global_scope)
    })
    .bind(address)?
    .run()
    .await
}
