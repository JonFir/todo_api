use super::{token, Claims};
use crate::configuration::AppState;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use log::{error, info, warn};
use std::future::{ready, Ready};
use std::sync::Arc;

enum TokenParseResult {
    Token(jsonwebtoken::TokenData<Claims>),
    Missing,
    DecodeError(crate::common::error::Error),
}

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
        let make_custom_result =
            |request: ServiceRequest, response: HttpResponse| -> Self::Future {
                let (request, _) = request.into_parts();
                let response = response.map_into_right_body();
                Box::pin(async { Ok(ServiceResponse::new(request, response)) })
            };

        let parse_token =
            |request: &ServiceRequest, jwt_secret: &str| -> TokenParseResult {
                let token_parts = request
                    .headers()
                    .get("Authorization")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .split(" ")
                    .collect::<Vec<&str>>();

                if token_parts.len() != 2
                    || !token_parts.first().unwrap_or(&"").eq(&"Bearer")
                {
                    return TokenParseResult::Missing;
                }
                let token = token_parts[1];
                let token = token::decode(token, jwt_secret);
                match token {
                    Ok(v) => TokenParseResult::Token(v),
                    Err(e) => TokenParseResult::DecodeError(e),
                }
            };

        let token =
            parse_token(&request, &self.app_state.environment.jwt_secret);

        match token {
            TokenParseResult::Token(v) => {
                info!("Success auth with uuid: {}", v.claims.sub);
                request.extensions_mut().insert(v);
                let result = self.service.call(request);
                Box::pin(async move {
                    let response = result.await?.map_into_left_body();
                    Ok(response)
                })
            }
            TokenParseResult::Missing => {
                warn!(
                    "Attempt access to path {} witn no token",
                    request.path()
                );
                make_custom_result(
                    request,
                    HttpResponse::Unauthorized().finish(),
                )
            }
            TokenParseResult::DecodeError(e) => {
                error!("JWT decode error with {}", e);
                make_custom_result(
                    request,
                    HttpResponse::InternalServerError().finish(),
                )
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
