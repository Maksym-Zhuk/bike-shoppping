use actix_web::{HttpMessage, HttpRequest, HttpResponse, Result, web};

use crate::{
    AppState, dto::auth::UserInfo, errors::AppErrors, services::user_service, utils::jwt::Claims,
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
        Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "unauthorized",
            "message": "No claims found"
        })))
    }
}
