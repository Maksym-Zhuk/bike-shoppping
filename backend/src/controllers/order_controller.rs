use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};

use crate::{
    dto::order::{CreateOrderDto, UpdateOrderDto},
    errors::{auth_error::AuthError, AppErrors, ErrorResponse},
    models::order::Order,
    services::order_service,
    utils::jwt::Claims,
    AppState,
};

#[utoipa::path(
    get,
    path = "/order/admin/orders",
    responses(
        (status = 200, description = "List of all orders", body = [Order]),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
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
    tag = "Orders",
    security(
        ("bearer_auth" = [])  
    )
)]
pub async fn get_all_orders(db: web::Data<AppState>) -> Result<HttpResponse, AppErrors> {
    let orders = order_service::get_all_orders(&db.mongo).await?;
    Ok(HttpResponse::Ok().json(orders))
}

#[utoipa::path(
    post,
    path = "/order/create",
    request_body = CreateOrderDto,
    responses(
        (status = 201, description = "Order created successfully", body = ErrorResponse, example = json!({
            "message": "Order created successfully"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 403, description = "Not enough rights", body = ErrorResponse,
            example = json!({
                "error": "insufficient_permissions",
                "message": "Necessary role: Admin"
            })
        ),
        (status = 404, description = "Some products not found", body = ErrorResponse, example = json!({
            "error": "not_found",
            "message": "Some product not found"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Orders",
    security(
        ("bearer_auth" = [])  
    )
)]
pub async fn create_order(
    db: web::Data<AppState>,
    new_order_data: web::Json<CreateOrderDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppErrors> {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let answer =
            order_service::create_order(&db.mongo, new_order_data, claims.sub.clone()).await?;
        Ok(HttpResponse::Created().json(serde_json::json!({
            "message": answer
        })))
    } else {
        Err(AppErrors::Auth(AuthError::Unauthorized))
    }
}

#[utoipa::path(
    get,
    path = "/order/{id}",
    params(
        ("id" = String, Path, description = "Order ID (UUID format)")
    ),
    responses(
        (status = 200, description = "Order found", body = Order),
        (status = 400, description = "Invalid UUID format", body = ErrorResponse, example = json!({
            "error": "invalid_uuid",
            "message": "Invalid UUID format"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 404, description = "Order not found", body = ErrorResponse, example = json!({
            "error": "not_found",
            "message": "Order not found"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Orders",
    security(
        ("bearer_auth" = [])  
    )
)]
pub async fn get_order(
    db: web::Data<AppState>,
    order_id: web::Path<String>,
) -> Result<HttpResponse, AppErrors> {
    let order = order_service::get_order(&db.mongo, &order_id).await?;
    Ok(HttpResponse::Ok().json(order))
}

#[utoipa::path(
    put,
    path = "/order/admin/update",
    request_body = UpdateOrderDto,
    responses(
        (status = 200, description = "Order updated successfully", body = ErrorResponse, example = json!({
            "message": "Order updated successfully"
        })),
        (status = 400, description = "Invalid UUID format", body = ErrorResponse, example = json!({
            "error": "invalid_uuid",
            "message": "Invalid UUID format"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 403, description = "Not enough rights", body = ErrorResponse,
            example = json!({
                "error": "insufficient_permissions",
                "message": "Necessary role: Admin"
            })
        ),
        (status = 404, description = "Order not found", body = ErrorResponse, example = json!({
            "error": "not_found",
            "message": "Order not found"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Orders",
    security(
        ("bearer_auth" = [])  
    )
)]
pub async fn update_order(
    db: web::Data<AppState>,
    new_order_data: web::Json<UpdateOrderDto>,
) -> Result<HttpResponse, AppErrors> {
    let answer = order_service::update_order(&db.mongo, new_order_data).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": answer
    })))
}

#[utoipa::path(
    delete,
    path = "/order/admin/delete/{id}",
    params(
        ("id" = String, Path, description = "Order ID (UUID format)")
    ),
    responses(
        (status = 200, description = "Order deleted successfully", body = ErrorResponse, example = json!({
            "message": "Order deleted successfully"
        })),
        (status = 400, description = "Invalid UUID format", body = ErrorResponse, example = json!({
            "error": "invalid_uuid",
            "message": "Invalid UUID format"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 403, description = "Not enough rights", body = ErrorResponse,
            example = json!({
                "error": "insufficient_permissions",
                "message": "Necessary role: Admin"
            })
        ),
        (status = 404, description = "Order not found", body = ErrorResponse, example = json!({
            "error": "not_found",
            "message": "Order not found"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Orders",
    security(
        ("bearer_auth" = [])  
    )
)]
pub async fn delete_order(
    db: web::Data<AppState>,
    order_id: web::Path<String>,
) -> Result<HttpResponse, AppErrors> {
    let answer = order_service::delete_order(&db.mongo, &order_id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": answer
    })))
}
