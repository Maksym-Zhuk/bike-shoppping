use crate::controllers::product_controller;
use actix_web::{HttpResponse, Responder, get, post, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product")
            .route("", web::get().to(product_controller::get_products))
            .route("", web::post().to(product_controller::create_product)),
    );
}
