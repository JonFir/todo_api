use argon2::{Config, ThreadMode, Variant, Version};

use crate::{common::errors::Error, features::auth::random_string};

pub fn new(password: &str) -> Result<String, Error> {
    let salt = random_string::new(128);
    static CONFIG: Config = Config {
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
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &CONFIG)
        .map_err(Error::from_parent)
}

pub fn verify(hash: &str, password: &str) -> Result<bool, Error> {
    argon2::verify_encoded(hash, password.as_bytes())
        .map_err(Error::from_parent)
}
