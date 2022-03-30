use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegisterPayload {
    pub login: String,
    pub password: String,
    pub email: String,
}