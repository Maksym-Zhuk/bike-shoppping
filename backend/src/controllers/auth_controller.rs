use actix_web::{HttpResponse, Responder, web};
use validator::Validate;

use crate::{
    AppState,
    dto::auth::{AuthResponse, LoginDto, RefreshTokenRequest, RefreshTokenResponse, RegisterDto},
    models::res::{ErrorResponse, MessageResponse},
    services::auth_service,
};

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterDto,
    responses(
        (status = 200, description = "Successful register", body = AuthResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn register(db: web::Data<AppState>, data: web::Json<RegisterDto>) -> impl Responder {
    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().json(MessageResponse {
            message: format!("Validation failed: {:?}", e),
        });
    }

    match auth_service::register(&db.mongo, data).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: err.to_string(),
            })
        }
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Successful register", body = AuthResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn login(db: web::Data<AppState>, data: web::Json<LoginDto>) -> impl Responder {
    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().json(MessageResponse {
            message: format!("Validation failed: {:?}", e),
        });
    }

    match auth_service::login(&db.mongo, data).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: err.to_string(),
            })
        }
    }
}

#[utoipa::path(
    post,
    path = "/auth/refresh_token",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Successful register", body = RefreshTokenResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn refresh_token(data: web::Json<RefreshTokenRequest>) -> impl Responder {
    match auth_service::refresh_token(data).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: err.to_string(),
            })
        }
    }
}
