use std::{borrow::Cow, fmt};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use super::{errors::Error, ResponsePayload};

#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum Code {
    UserExist = 1,
    UserNotFound = 2,
    AccessTokenMissing = 3,
    Internal = 500,
}

impl Code {
    fn raw_value(self) -> u64 {
        self as u64
    }
}

#[derive(Debug)]
pub struct ErrorMeta {
    code: Code,
    status_code: StatusCode,
    message: Cow<'static, str>,
}

impl ErrorMeta {
    pub const USER_EXIST: ErrorMeta = ErrorMeta {
        code: Code::UserExist,
        status_code: StatusCode::CONFLICT,
        message: Cow::Borrowed("User with same login or email alredy exist"),
    };
    pub const USER_NOT_FOUND: ErrorMeta = ErrorMeta {
        code: Code::UserNotFound,
        status_code: StatusCode::FORBIDDEN,
        message: Cow::Borrowed("Bad credentials"),
    };
    pub const ACCESS_TOKEN_MISSING: ErrorMeta = ErrorMeta {
        code: Code::AccessTokenMissing,
        status_code: StatusCode::UNAUTHORIZED,
        message: Cow::Borrowed("Incorrect access token"),
    };
    pub const INTERNAL: ErrorMeta = ErrorMeta {
        code: Code::Internal,
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: Cow::Borrowed("Internal server error"),
    };
}

impl Into<ErrorResponse> for ErrorMeta {
    fn into(self) -> ErrorResponse {
        ErrorResponse {
            meta: self,
            parent: None,
        }
    }
}

#[derive(Debug)]
pub struct ErrorResponse {
    pub meta: ErrorMeta,
    pub parent: Option<Error>,
}

impl std::error::Error for ErrorResponse {}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "code: {:}, message: {:}, parent: {:?}",
            self.meta.code.raw_value(),
            self.meta.message,
            self.parent
        )
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.meta.status_code
    }

    fn error_response(&self) -> HttpResponse {
        let body = ResponsePayload::error(
            self.meta.code.raw_value(),
            self.meta.message.clone(),
        );
        HttpResponse::build(self.status_code()).json(body)
    }
}
