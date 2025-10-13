use actix_web::{HttpResponse, Responder};

pub mod order_controller;
pub mod product_controller;

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not found!")
}
