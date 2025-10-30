use crate::utils::jwt;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
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
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        match token {
            Some(token) => match jwt::validate_token(token.to_string()) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims.clone());
                    req.extensions_mut().insert(claims.role.clone());

                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res.map_into_left_body())
                    })
                }
                Err(err) => {
                    let (req, _) = req.into_parts();
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "error": "invalid_token",
                            "message": format!("Invalid token: {}", err)
                        }))
                        .map_into_right_body();

                    Box::pin(async move { Ok(ServiceResponse::new(req, response)) })
                }
            },
            None => {
                let (req, _) = req.into_parts();
                let response = HttpResponse::Unauthorized()
                    .json(serde_json::json!({
                        "error": "missing_token",
                        "message": "Missing or invalid Authorization header"
                    }))
                    .map_into_right_body();

                Box::pin(async move { Ok(ServiceResponse::new(req, response)) })
            }
        }
    }
}
