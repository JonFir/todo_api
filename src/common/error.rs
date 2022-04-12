use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::fmt;

use super::ResponsePayload;

#[derive(Debug)]
pub enum ErrorKinde {
    UserExist,
    UserNotFound,
    Internal,
}
#[derive(Debug)]
pub struct ErrorMeta {
    kinde: ErrorKinde,
    status_code: StatusCode,
    code: u64,
    message: &'static str,
}
#[derive(Debug)]
pub struct Error {
    meta: ErrorMeta,
    parent: Option<Box<dyn std::error::Error>>,
}

impl ErrorMeta {
    pub const USER_EXIST: ErrorMeta = ErrorMeta {
        kinde: ErrorKinde::UserExist,
        status_code: StatusCode::CONFLICT,
        code: 1,
        message: "User alredy exist",
    };
    pub const USER_NOT_FOUND: ErrorMeta = ErrorMeta {
        kinde: ErrorKinde::UserNotFound,
        status_code: StatusCode::CONFLICT,
        code: 2,
        message: "User not found",
    };
    pub const INTERNAL: ErrorMeta = ErrorMeta {
        kinde: ErrorKinde::Internal,
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        code: 500,
        message: "Internal server error",
    };
}

impl From<ErrorMeta> for Error {
    fn from(meta: ErrorMeta) -> Self {
        Error { meta, parent: None }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "message: {:}, parent: {:?}",
            self.meta.message, self.parent
        )
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.meta.status_code
    }

    fn error_response(&self) -> HttpResponse {
        let body = ResponsePayload::error(
            self.meta.code,
            String::from(self.meta.message),
        );
        HttpResponse::build(self.status_code()).json(body)
    }
}

impl Error {
    pub fn from_db_error(
        transform: fn(std::borrow::Cow<str>) -> Option<Error>,
    ) -> impl Fn(sqlx::error::Error) -> Error {
        move |error| {
            error
                .as_database_error()
                .and_then(|e| e.code())
                .and_then(transform)
                .unwrap_or(Error::from_parent(error))
        }
    }

    pub fn from_parent<E: 'static + std::error::Error>(error: E) -> Error {
        Error {
            meta: ErrorMeta::INTERNAL,
            parent: Some(Box::new(error)),
        }
    }
}
