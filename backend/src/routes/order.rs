use crate::controllers::order_controller;
use actix_web::{Scope, web};

pub fn init() -> Scope {
    web::scope("/order")
        .route("/orders", web::get().to(order_controller::get_all_orders))
        .route("/create", web::post().to(order_controller::create_order))
        .route("/{id}", web::get().to(order_controller::get_order))
        .route("/update", web::put().to(order_controller::update_order))
        .route(
            "/delete/{id}",
            web::delete().to(order_controller::delete_order),
        )
}
