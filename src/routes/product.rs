use crate::controllers::product_controller;
use actix_web::{Scope, web};

pub fn init() -> Scope {
    web::scope("/product")
        .route("", web::get().to(product_controller::get_all_products))
        .route("", web::post().to(product_controller::create_product))
}
