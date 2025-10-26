use crate::{
    controllers::order_controller,
    middleware::{auth::JwtMiddleware, permissions::PermissionCheck},
    models::role::Role,
};
use actix_web::web;

pub fn init() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/order")
        .service(
            web::scope("/admin")
                .wrap(PermissionCheck::new(Role::Admin))
                .wrap(JwtMiddleware)
                .route("/orders", web::get().to(order_controller::get_all_orders))
                .route("/update", web::put().to(order_controller::update_order))
                .route(
                    "/delete/{id}",
                    web::delete().to(order_controller::delete_order),
                ),
        )
        .service(
            web::scope("")
                .wrap(JwtMiddleware)
                .route("/create", web::post().to(order_controller::create_order))
                .route("/{id}", web::get().to(order_controller::get_order)),
        )
}
