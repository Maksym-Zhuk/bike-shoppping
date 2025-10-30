use actix_web::{web, Scope};

use crate::controllers::auth_controller;

pub fn init() -> Scope {
    web::scope("/auth")
        .route("/register", web::post().to(auth_controller::register))
        .route("/login", web::post().to(auth_controller::login))
        .route(
            "/refresh_token",
            web::post().to(auth_controller::refresh_token),
        )
}
