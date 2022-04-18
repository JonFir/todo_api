use crate::common::{
    configuration::AppState,
    error_response::{ErrorMeta, ErrorResponse},
};

use super::token;

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::sync::Arc;

pub struct JwtService<S> {
    service: S,
    app_state: Arc<AppState>,
}

impl<S, B> Service<ServiceRequest> for JwtService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let token = token::extract_from_headers(
            request.headers(),
            &self.app_state.environment.jwt_secret,
        );

        match token {
            Ok(v) => {
                request.extensions_mut().insert(v);
                let result = self.service.call(request);
                Box::pin(async move {
                    let response = result.await?.map_into_left_body();
                    Ok(response)
                })
            }
            Err(e) => {
                let (request, _) = request.into_parts();
                let response = ErrorResponse {
                    meta: ErrorMeta::ACCESS_TOKEN_MISSING,
                    parent: e.into(),
                }
                .error_response()
                .map_into_right_body();
                Box::pin(async { Ok(ServiceResponse::new(request, response)) })
            }
        }
    }
}

pub struct Middleware {
    pub app_state: Arc<AppState>,
}

impl<S, B> Transform<S, ServiceRequest> for Middleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let service = JwtService {
            service,
            app_state: self.app_state.clone(),
        };
        ready(Ok(service))
    }
}
