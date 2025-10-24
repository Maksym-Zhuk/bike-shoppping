use actix_web::{HttpResponse, Responder};

pub mod auth_controller;
pub mod order_controller;
pub mod product_controller;
pub mod user_controller;

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not found!")
}
