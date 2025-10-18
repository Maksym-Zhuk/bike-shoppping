use crate::controllers::product_controller;
use actix_web::{Scope, web};

pub fn init() -> Scope {
    web::scope("/product")
        .route(
            "/most_advantageous",
            web::get().to(product_controller::get_most_advantageous),
        )
        .route(
            "/products",
            web::get().to(product_controller::get_all_products),
        )
        .route(
            "/create",
            web::post().to(product_controller::create_product),
        )
        .route("/{id}", web::get().to(product_controller::get_product))
        .route("/update", web::put().to(product_controller::update_product))
        .route(
            "/delete/{id}",
            web::delete().to(product_controller::delete_product),
        )
}
