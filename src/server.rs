use std::sync::Arc;

use actix_web::{web, App, HttpServer, get, Responder, HttpResponse};
use configuration::AppState;

use crate::{api, middleware};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn run(address: (String, u16), state: Arc<AppState>) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&state)))
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
    .bind(address)?
    .run()
    .await
}