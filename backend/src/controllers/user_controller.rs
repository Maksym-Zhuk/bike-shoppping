use actix_web::{HttpMessage, HttpRequest, HttpResponse, Result, web};
use validator::Validate;

use crate::{
    AppState,
    dto::{auth::UserInfo, user::UpdateUserDto},
    errors::{AppErrors, auth_error::AuthError},
    models::res::MessageResponse,
    services::user_service,
    utils::jwt::Claims,
};

#[utoipa::path(
    get,
    path = "/user/me",
    responses(
        (status = 200, description = "User info retrieved successfully", body = UserInfo),
        (status = 401, description = "Unauthorized", body = inline(Object), example = json!({
            "error": "unauthorized",
            "message": "No claims found"
        })),
        (status = 401, description = "Unauthorized", body = inline(Object), example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 500, description = "Internal server error", body = inline(Object), example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn me(db: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, AppErrors> {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let res = user_service::me(&db.mongo, claims.sub.clone()).await?;
        Ok(HttpResponse::Ok().json(res))
    } else {
        Err(AppErrors::Auth(AuthError::Unauthorized))
    }
}

#[utoipa::path(
    put,
    path = "/user/update",
    request_body = UpdateUserDto,
    responses(
        (status = 200, description = "User info retrieved successfully", body = MessageResponse),
        (status = 400, description = "Validation failed", body = inline(Object), example = json!({
            "error": "validation_error",
            "message": "Validation failed"
        })),
        (status = 401, description = "Invalid credentials", body = inline(Object), example = json!({
            "error": "invalid_credentials",
            "message": "Invalid email, password or name"
        })),
        (status = 401, description = "Unauthorized", body = inline(Object), example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 500, description = "Internal server error", body = inline(Object), example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_user(
    db: web::Data<AppState>,
    req: HttpRequest,
    new_data: web::Json<UpdateUserDto>,
) -> Result<HttpResponse, AppErrors> {
    if let Err(e) = new_data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "validation_error",
            "message": format!("Validation failed: {:?}", e)
        })));
    }

    if let Some(claims) = req.extensions().get::<Claims>() {
        let res = user_service::update_user(&db.mongo, claims.sub.clone(), new_data).await?;
        Ok(HttpResponse::Ok().json(MessageResponse { message: res }))
    } else {
        Err(AppErrors::Auth(AuthError::Unauthorized))
    }
}

#[utoipa::path(
    delete,
    path = "/user/delete",
    responses(
        (status = 200, description = "User info retrieved successfully", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = inline(Object), example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 404, description = "Product not found", body = inline(Object), example = json!({
            "error": "not_found",
            "message": "Product not found"
        })),
        (status = 500, description = "Internal server error", body = inline(Object), example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_user(
    db: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppErrors> {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let res = user_service::delete_user(&db.mongo, claims.sub.clone()).await?;
        Ok(HttpResponse::Ok().json(MessageResponse { message: res }))
    } else {
        Err(AppErrors::Auth(AuthError::Unauthorized))
    }
}
