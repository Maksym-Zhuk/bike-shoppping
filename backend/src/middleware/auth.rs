use std::future::{Ready, ready};

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;

use crate::utils::jwt;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
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
        let auth_header = req.headers().get("Authorization");
        let token = match auth_header {
            Some(header_value) => match header_value.to_str() {
                Ok(header_str) => {
                    if header_str.starts_with("Bearer ") {
                        Some(header_str[7..].to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            None => None,
        };

        let token = match token {
            Some(t) => t,
            None => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized(
                        "Missing or invalid Authorization header",
                    ))
                });
            }
        };

        match jwt::validate_token(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);

                let fut = self.service.call(req);

                Box::pin(async move {
                    let res = fut.await?;

                    Ok(res)
                })
            }
            Err(err) => Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(format!(
                    "Invalid token: {}",
                    err
                )))
            }),
        }
    }
}
