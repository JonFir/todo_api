use super::payloads::RegisterPayload;
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[post("/register")]
pub async fn register(payload: web::Json<RegisterPayload>) -> impl Responder {
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    let config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 15360,
        time_cost: 10,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 128,
    };
    let hash = argon2::hash_encoded(payload.password.as_bytes(), salt.as_bytes(), &config).unwrap();
    HttpResponse::Ok().body(hash + "" + &salt)
}

#[post("/login")]
pub async fn login(payload: web::Json<RegisterPayload>) -> impl Responder {
    HttpResponse::Ok().body("login")
}
