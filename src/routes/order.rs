use crate::controllers::order_controller;
use actix_web::{Scope, web};

pub fn init() -> Scope {
    web::scope("/order").route(
        "/create_order",
        web::post().to(order_controller::create_order),
    )
}
