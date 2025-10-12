use actix_web::{HttpResponse, Responder, web};
use mongodb::Database;

use crate::{models::order::CreateOrderDto, services::order_service};

#[utoipa::path(
    post,
    path = "/order/create_order",
    request_body = CreateOrderDto,
    responses(
        (status = 201, description = "Order created successfully"),
        (status = 404, description = "Some products not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Orders"
)]
pub async fn create_order(
    db: web::Data<Database>,
    new_order_data: web::Json<CreateOrderDto>,
) -> impl Responder {
    match order_service::create_order(&db, new_order_data).await {
        Ok(answer) => HttpResponse::Created().body(answer),
        Err(err) => {
            eprintln!("{}", err);
            let msg = format!("{}", err);
            if msg.contains("Some products not found") {
                return HttpResponse::NotFound().body(msg);
            }

            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
