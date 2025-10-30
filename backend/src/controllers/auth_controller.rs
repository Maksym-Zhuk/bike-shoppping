use actix_web::{HttpResponse, Result, web};
use validator::Validate;

use crate::{
    dto::auth::{LoginDto, RefreshTokenRequest, RegisterDto, UserInfo}, errors::{AppErrors, ErrorResponse}, models::res::MessageResponse, services::auth_service, AppState
};

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterDto,
    responses(
        (   
            status = 200, 
            description = "Successful registration", 
            body = UserInfo, 
            headers(
                ("X-Access-Token" = String, description = "JWT access token for authentication"),
                ("X-Refresh-Token" = String, description = "JWT refresh token for obtaining new access token")
            )),
        (status = 400, description = "Validation failed", body = ErrorResponse, example = json!({
            "error": "validation_error",
            "message": "Validation failed"
        })),
        (status = 409, body = ErrorResponse, example = json!({
            "error": "duplicate_key",
            "message": "Email already exists"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Auth"
)]
pub async fn register(
    db: web::Data<AppState>,
    data: web::Json<RegisterDto>,
) -> Result<HttpResponse, AppErrors> {
    if let Err(e) = data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "validation_error",
            "message": format!("Validation failed: {:?}", e)
        })));
    }

    let res = auth_service::register(&db.mongo, data).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("X-Access-Token", res.access_token))
        .insert_header(("X-Refresh-Token", res.refresh_token))
        .json(res.user))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginDto,
    responses(
        (
            status = 200, 
            description = "Successful login", 
            body = UserInfo,
            headers(
                ("X-Access-Token" = String, description = "JWT access token for authentication"),
                ("X-Refresh-Token" = String, description = "JWT refresh token for obtaining new access token")
            )
        ),
        (status = 400, description = "Validation failed", body = ErrorResponse, example = json!({
            "error": "validation_error",
            "message": "Validation failed"
        })),
        (status = 401, description = "Invalid credentials", body = ErrorResponse, example = json!({
            "error": "invalid_credentials",
            "message": "Invalid email or password"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Auth"
)]
pub async fn login(
    db: web::Data<AppState>,
    data: web::Json<LoginDto>,
) -> Result<HttpResponse, AppErrors> {
    if let Err(e) = data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "validation_error",
            "message": format!("Validation failed: {:?}", e)
        })));
    }

    let res = auth_service::login(&db.mongo, data).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("X-Access-Token", res.access_token))
        .insert_header(("X-Refresh-Token", res.refresh_token))
        .json(res.user))
}

#[utoipa::path(
    post,
    path = "/auth/refresh_token",
    request_body = RefreshTokenRequest,
    responses(
        (
            status = 200, 
            description = "Token refreshed successfully", 
            body = MessageResponse, 
            headers(
                ("X-Access-Token" = String, description = "JWT access token for authentication"),
            )),
        (status = 401, description = "Invalid refresh token", body = ErrorResponse, example = json!({
            "error": "invalid_refresh_token",
            "message": "Invalid refresh token"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        }))
    ),
    tag = "Auth"
)]
pub async fn refresh_token(
    data: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse, AppErrors> {
    let res = auth_service::refresh_token(data).await?;
    Ok(HttpResponse::Ok().insert_header(("X-Access-Token", res)).json(MessageResponse {
        message: "Token refreshed successfully".to_string()
    }))
}
