use std::{env, time::Duration};

use crate::{
    api_docs::ApiDoc,
    db::{mongo::init_db, redis::init_redis},
};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use log::info;
use redis::aio::ConnectionManager;
use utoipa::OpenApi;

mod api_docs;
mod config;
mod controllers;
mod db;
mod dto;
mod errors;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

struct AppState {
    mongo: mongodb::Database,
    redis: ConnectionManager,
}

#[actix_web::main]
async fn main() {
    config::init();
    let mongo: mongodb::Database = init_db().await;
    let redis = init_redis().await.expect("Failed to connect to Redis");
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let openapi: utoipa::openapi::OpenApi = ApiDoc::openapi();
    let governor_conf = GovernorConfigBuilder::default()
        .period(Duration::from_millis(50))
        .burst_size(30)
        .finish()
        .unwrap();

    let state = web::Data::new(AppState { mongo, redis });

    info!("Server started in the port: {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        let configure = web::scope("/api")
            .wrap(cors)
            .wrap(Governor::new(&governor_conf))
            .configure(routes::init);
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(configure)
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
