use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};
use validator::Validate;

use crate::{
    dto::{auth::UserInfo, user::UpdateUserDto},
    errors::{auth_error::AuthError, AppErrors, ErrorResponse},
    models::{app::AppState, order::Order, res::MessageResponse},
    services::user_service,
    utils::jwt::Claims,
};

#[utoipa::path(
    get,
    path = "/user/me",
    responses(
        (status = 200, description = "User info retrieved successfully", body = UserInfo),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "invalid_token",
            "message": "Invalid token: "
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
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
        (status = 200, description = "User updated successfully", body = MessageResponse),
        (status = 400, description = "Validation failed", body = ErrorResponse, example = json!({
            "error": "validation_error",
            "message": "Validation failed"
        })),
        (status = 400, description = "Invalid credentials", body = ErrorResponse, example = json!({
            "error": "invalid_credentials",
            "message": "Invalid email, password or name"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "invalid_token",
            "message": "Invalid token: "
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
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
        (status = 200, description = "User deleted successfully", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "invalid_token",
            "message": "Invalid token:"
        })),
        (status = 404, description = "User not found", body = ErrorResponse, example = json!({
            "error": "not_found",
            "message": "User not found"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
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

#[utoipa::path(
    get,
    path = "/user/admin/users",
    responses(
        (status = 200, description = "Users info retrieved successfully", body = [UserInfo]),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "invalid_token",
            "message": "Invalid token:"
        })),
        (status = 403, description = "Not enough rights", body = ErrorResponse,
            example = json!({
                "error": "insufficient_permissions",
                "message": "Necessary role: Admin"
            })
        ),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_all_users(db: web::Data<AppState>) -> Result<HttpResponse, AppErrors> {
    let res = user_service::get_all_users(&db.mongo).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    get,
    path = "/user/my_orders",
    responses(
        (status = 200, body = [Order]),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "unauthorized",
            "message": "No claims found"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_my_orders(
    db: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppErrors> {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let res = user_service::get_my_orders(&db.mongo, claims.sub.clone()).await?;
        Ok(HttpResponse::Ok().json(res))
    } else {
        Err(AppErrors::Auth(AuthError::Unauthorized))
    }
}
