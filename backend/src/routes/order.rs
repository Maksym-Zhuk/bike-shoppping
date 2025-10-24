use crate::{controllers::order_controller, middleware::auth::JwtMiddleware};
use actix_web::web;

pub fn init() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/order")
        .wrap(JwtMiddleware)
        .route("/orders", web::get().to(order_controller::get_all_orders))
        .route("/create", web::post().to(order_controller::create_order))
        .route("/{id}", web::get().to(order_controller::get_order))
        .route("/update", web::put().to(order_controller::update_order))
        .route(
            "/delete/{id}",
            web::delete().to(order_controller::delete_order),
        )
}
