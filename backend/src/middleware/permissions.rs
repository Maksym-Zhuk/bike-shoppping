use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;

use crate::models::role::Role;
use std::future::{Ready, ready};

pub struct PermissionCheck {
    required_role: Role,
}

impl PermissionCheck {
    pub fn new(required_role: Role) -> Self {
        PermissionCheck { required_role }
    }
}

impl<S, B> Transform<S, ServiceRequest> for PermissionCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionCheckService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionCheckService {
            service,
            required_role: self.required_role.clone(),
        }))
    }
}

pub struct PermissionCheckService<S> {
    service: S,
    required_role: Role,
}

impl<S, B> Service<ServiceRequest> for PermissionCheckService<S>
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
        let user_role = req.extensions().get::<Role>().cloned();
        let required_role = self.required_role.clone();

        match user_role {
            Some(role) if role == required_role => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            }
            _ => {
                let (req, _) = req.into_parts();
                let response = HttpResponse::Forbidden()
                    .json(serde_json::json!({
                        "error": "insufficient_permissions",
                        "message": format!("Necessary role: {:?}", required_role)
                    }))
                    .map_into_right_body();

                Box::pin(async move { Ok(ServiceResponse::new(req, response)) })
            }
        }
    }
}
