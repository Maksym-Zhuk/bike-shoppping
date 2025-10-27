use actix_web::web;

use crate::{
    controllers::user_controller,
    middleware::{auth::JwtMiddleware, permissions::PermissionCheck},
    models::role::Role,
};

pub fn init() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/user")
        .service(
            web::scope("/admin")
                .wrap(PermissionCheck::new(Role::Admin))
                .wrap(JwtMiddleware)
                .route("/users", web::get().to(user_controller::get_all_users)),
        )
        .service(
            web::scope("")
                .wrap(JwtMiddleware)
                .route("/me", web::get().to(user_controller::me))
                .route("/update", web::put().to(user_controller::update_user))
                .route("/delete", web::delete().to(user_controller::delete_user))
                .route("/my_orders", web::get().to(user_controller::get_my_orders)),
        )
}
