use std::env;

use crate::{api_docs::ApiDoc, db::mongo_client::init_db};
use actix_web::{
    App, HttpServer,
    web::{self},
};
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api_docs;
mod config;
mod controllers;
mod db;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() {
    config::init();
    let db = init_db().await;
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let openapi = ApiDoc::openapi();

    info!("Server started in the port: {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(web::scope("/api").configure(routes::product::config))
            .service(
                utoipa_swagger_ui::SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone()),
            )
            .default_service(web::to(handlers::not_found))
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .run()
    .await
    .unwrap()
}
