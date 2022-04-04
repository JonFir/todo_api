use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::sync::Arc;

use crate::{api::auth::handlers::Claims, AppState};

pub struct Middleware<S> {
    service: S,
    app_state: Arc<AppState>,
}

impl<S, B> Service<ServiceRequest> for Middleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("key").unwrap().to_str().ok().unwrap();
        let token = jsonwebtoken::decode::<Claims>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(self.app_state.environment.jwt_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .unwrap();
        req.extensions_mut().insert(token);
        let result = self.service.call(req);
        Box::pin(async move {
            let response = result.await?;
            Ok(response)
        })
    }
}

pub struct Transformer {
    pub app_state: Arc<AppState>,
}

impl<S, B> Transform<S, ServiceRequest> for Transformer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = Middleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Middleware {
            service,
            app_state: self.app_state.clone(),
        }))
    }
}
