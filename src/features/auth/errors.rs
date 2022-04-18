use crate::common;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidTokenData(jsonwebtoken::errors::Error),
    InvalidToken(jsonwebtoken::errors::Error),
    MissingToken,
    InvalidClaimsUuid,
    PasswordHashingFail(argon2::Error),
    PasswordVerifyFail(argon2::Error),
    DbUserAlreadyExist(sqlx::error::Error),
    DbUserCreateFail(sqlx::error::Error),
    DbUserNotFound(sqlx::error::Error),
    DbUpdateRefreshTokenFail(sqlx::error::Error),
    IncorectLogin,
    IncorectPassword,
    UserNotFound,
}
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self,)
    }
}

impl Into<common::errors::Error> for Error {
    fn into(self) -> common::errors::Error {
        common::errors::Error::Auth(self)
    }
}

impl Into<Option<common::errors::Error>> for Error {
    fn into(self) -> Option<common::errors::Error> {
        Some(common::errors::Error::Auth(self))
    }
}
