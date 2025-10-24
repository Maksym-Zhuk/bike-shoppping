use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};

use crate::{
    AppState, dto::auth::UserInfo, models::res::ErrorResponse, services::user_service,
    utils::jwt::Claims,
};

#[utoipa::path(
    get,
    path = "/user/me",
    responses(
        (status = 200, description = "Successful register", body = UserInfo),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])  
    )
)]
pub async fn me(db: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        match user_service::me(&db.mongo, claims.sub.clone()).await {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(err) => {
                eprintln!("{}", err);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    message: err.to_string(),
                })
            }
        }
    } else {
        HttpResponse::Unauthorized().body("No claims found")
    }
}
