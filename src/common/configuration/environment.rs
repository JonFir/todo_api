use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Environment {
    pub url: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_duration: i64,
}

impl Environment {
    pub fn load() -> Result<Environment, envy::Error> {
        dotenv::dotenv().ok();
        envy::from_env::<Environment>()
    }

    pub fn socket_addrs(&self) -> (String, u16) {
        (self.url.to_owned(), self.port)
    }
}
