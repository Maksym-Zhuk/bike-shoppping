use actix_web::web;

use crate::{controllers::user_controller, middleware::auth::JwtMiddleware};

pub fn init() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/user")
        .wrap(JwtMiddleware)
        .route("/me", web::get().to(user_controller::me))
        .route("/update", web::put().to(user_controller::update_user))
        .route("/delete", web::delete().to(user_controller::delete_user))
}
