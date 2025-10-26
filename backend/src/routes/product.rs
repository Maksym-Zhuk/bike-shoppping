use crate::{
    controllers::product_controller,
    middleware::{auth::JwtMiddleware, permissions::PermissionCheck},
    models::role::Role,
};
use actix_web::{Scope, web};

pub fn init() -> Scope {
    web::scope("/product")
        .service(
            web::scope("")
                .wrap(JwtMiddleware)
                .route(
                    "/most_advantageous",
                    web::get().to(product_controller::get_most_advantageous),
                )
                .route(
                    "/products",
                    web::get().to(product_controller::get_all_products),
                )
                .route("/{id}", web::get().to(product_controller::get_product)),
        )
        .service(
            web::scope("")
                .wrap(JwtMiddleware)
                .wrap(PermissionCheck::new(Role::Admin))
                .route(
                    "/create",
                    web::post().to(product_controller::create_product),
                )
                .route("/update", web::put().to(product_controller::update_product))
                .route(
                    "/delete/{id}",
                    web::delete().to(product_controller::delete_product),
                ),
        )
}
