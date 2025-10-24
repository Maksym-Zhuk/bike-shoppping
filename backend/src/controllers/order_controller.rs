use actix_web::{HttpResponse, Responder, web};

use crate::{
    AppState,
    dto::order::{CreateOrderDto, UpdateOrderDto},
    models::{
        order::Order,
        res::{ErrorResponse, MessageResponse},
    },
    services::order_service,
};

#[utoipa::path(
    get,
    path = "/order/orders",
    responses(
        (status = 200, description = "List of all orders", body = [Order]),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Orders"
)]
pub async fn get_all_orders(db: web::Data<AppState>) -> impl Responder {
    match order_service::get_all_orders(&db.mongo).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[utoipa::path(
    post,
    path = "/order/create",
    request_body = CreateOrderDto,
    responses(
        (status = 201, description = "Order created successfully", body = MessageResponse),
        (status = 404, description = "Some products not found", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Orders"
)]
pub async fn create_order(
    db: web::Data<AppState>,
    new_order_data: web::Json<CreateOrderDto>,
) -> impl Responder {
    match order_service::create_order(&db.mongo, new_order_data).await {
        Ok(answer) => HttpResponse::Created().json(MessageResponse { message: answer }),
        Err(err) => {
            eprintln!("{}", err);
            let msg = format!("{}", err);
            if msg.contains("Some products not found") {
                return HttpResponse::NotFound().json(MessageResponse { message: msg });
            }

            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[utoipa::path(
    get,
    path = "/order/{id}",
    params(
        ("id" = String, Path, description = "Order ID")
    ),
    responses(
        (status = 200, description = "Order found", body = Order),
        (status = 404, description = "Order not found", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Orders"
)]
pub async fn get_order(db: web::Data<AppState>, order_id: web::Path<String>) -> impl Responder {
    match order_service::get_order(&db.mongo, &order_id).await {
        Ok(Some(order)) => HttpResponse::Ok().json(order),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Order not found".to_string(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[utoipa::path(
    put,
    path = "/order/update",
    request_body = UpdateOrderDto,
    responses(
        (status = 200, description = "Order updated successfully", body = MessageResponse),
        (status = 404, description = "Order not found", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Orders"
)]
pub async fn update_order(
    db: web::Data<AppState>,
    new_order_data: web::Json<UpdateOrderDto>,
) -> impl Responder {
    match order_service::update_order(&db.mongo, new_order_data).await {
        Ok(Some(answer)) => HttpResponse::Ok().json(MessageResponse { message: answer }),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Order not found".to_string(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[utoipa::path(
    delete,
    path = "/order/delete/{id}",
    params(
        ("id" = String, Path, description = "Order ID")
    ),
    responses(
        (status = 200, description = "Order deleted successfully", body = MessageResponse),
        (status = 404, description = "Order not found", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Orders"
)]
pub async fn delete_order(db: web::Data<AppState>, order_id: web::Path<String>) -> impl Responder {
    match order_service::delete_order(&db.mongo, &order_id).await {
        Ok(Some(answer)) => HttpResponse::Ok().json(MessageResponse { message: answer }),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Order not found".to_string(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}
