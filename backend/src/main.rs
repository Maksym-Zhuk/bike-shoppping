use std::{env, time::Duration};

use crate::{api_docs::ApiDoc, db::mongo_client::init_db};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    App, HttpServer,
    web::{self},
};
use log::info;
use utoipa::OpenApi;

mod api_docs;
mod config;
mod controllers;
mod db;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() {
    config::init();
    let db = init_db().await;
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let openapi: utoipa::openapi::OpenApi = ApiDoc::openapi();
    let governor_conf = GovernorConfigBuilder::default()
        .period(Duration::from_millis(50))
        .burst_size(30)
        .finish()
        .unwrap();

    info!("Server started in the port: {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8081")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(
                web::scope("/api")
                    .wrap(cors)
                    .wrap(Governor::new(&governor_conf))
                    .configure(routes::init),
            )
            .service(
                utoipa_swagger_ui::SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone()),
            )
            .default_service(web::to(controllers::not_found))
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .run()
    .await
    .unwrap()
}
