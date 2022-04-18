use std::fmt;

#[derive(Debug)]
pub enum Error {
    Auth(crate::features::auth::errors::Error),
}
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code: {:}", self,)
    }
}
