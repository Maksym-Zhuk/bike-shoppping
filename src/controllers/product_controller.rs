use actix_web::{HttpResponse, Responder};

pub async fn get_products() -> impl Responder {
    HttpResponse::Ok().json("Get all bikes")
}

pub async fn create_product() -> impl Responder {
    HttpResponse::Ok().json("Create bike")
}
