use actix_web::{body::BoxBody, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseEmptyData;

#[derive(Serialize)]
pub struct ResponsePayload<Data: Serialize> {
    pub error: u64,
    pub message: String,
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
    pub fn error(error: u64, message: String) -> Self {
        ResponsePayload {
            error,
            message,
            data: ResponseEmptyData {},
        }
    }

    pub fn succes_and_empty(message: String) -> Self {
        ResponsePayload {
            error: 0,
            message,
            data: ResponseEmptyData {},
        }
    }
}

impl<Data: Serialize> ResponsePayload<Data> {
    pub fn succes(message: String, data: Data) -> Self {
        ResponsePayload {
            error: 0,
            message,
            data,
        }
    }
}
