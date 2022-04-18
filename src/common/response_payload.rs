use std::borrow::Cow;

use actix_web::{body::BoxBody, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseEmptyData;

#[derive(Serialize)]
pub struct ResponsePayload<Data: Serialize> {
    pub error: u64,
    pub message: Cow<'static, str>,
    pub data: Data,
}

impl<Data> Responder for ResponsePayload<Data>
where
    Data: Serialize,
{
    type Body = BoxBody;

    fn respond_to(
        self,
        req: &actix_web::HttpRequest,
    ) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self).respond_to(req)
    }
}

impl ResponsePayload<ResponseEmptyData> {
    pub fn error<S>(error: u64, message: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        ResponsePayload {
            error,
            message: message.into(),
            data: ResponseEmptyData {},
        }
    }

    pub fn succes_and_empty<S>(message: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        ResponsePayload {
            error: 0,
            message: message.into(),
            data: ResponseEmptyData {},
        }
    }
}

impl<Data: Serialize> ResponsePayload<Data> {
    pub fn succes<S>(message: S, data: Data) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        ResponsePayload {
            error: 0,
            message: message.into(),
            data,
        }
    }
}
