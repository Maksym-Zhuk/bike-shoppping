use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    test, web, App,
};
use mongodb::{Client, Database};
use redis::aio::ConnectionManager;
use serde_json::json;
use uuid::Uuid;

use bike_shopping_backend::{
    models::app::AppState, routes, utils::jwt::generate_access_token, Role,
};

pub async fn setup_test_db() -> Database {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to connect to MongoDB");

    let db_name = format!("test_db_{}", Uuid::new_v4());
    let db = client.database(&db_name);

    setup_indexes(&db).await;

    db
}

pub async fn setup_indexes(db: &Database) {
    use mongodb::{bson::doc, options::IndexOptions, IndexModel};

    db.collection::<mongodb::bson::Document>("users")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
        )
        .await
        .ok();
}

pub async fn teardown_test_db(db: &Database) {
    db.drop().await.ok();
}

pub async fn setup_test_redis() -> ConnectionManager {
    let client =
        redis::Client::open("redis://localhost:6379").expect("Failed to create Redis client");

    ConnectionManager::new(client)
        .await
        .expect("Fail to connect to Redis")
}

pub async fn clear_test_redis(redis: &mut ConnectionManager) {
    use redis::AsyncCommands;

    let _: () = redis.flushdb().await.expect("Failed to flush Redis");
}

pub async fn clear_test_redis_prefix(redis: &mut ConnectionManager, prefix: &str) {
    use redis::AsyncCommands;

    let pattern = format!("{}*", prefix);
    let keys: Vec<String> = redis.keys(&pattern).await.unwrap_or_default();

    if !keys.is_empty() {
        let _: () = redis.del(keys).await.expect("Failed to delete keys");
    }
}

pub async fn create_test_app(
    db: Database,
    redis: ConnectionManager,
) -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    let state = web::Data::new(AppState { mongo: db, redis });

    test::init_service(
        App::new()
            .app_data(state.clone())
            .service(web::scope("/api").configure(routes::init)),
    )
    .await
}

pub async fn generate_test_admin_token() -> Result<String, bike_shopping_backend::AppErrors> {
    generate_access_token(String::from(Uuid::new_v4()), Role::Admin)
}

pub async fn register_test_user(
    app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
    name: &str,
    email: &str,
    password: &str,
) -> ServiceResponse {
    let register_payload = json!({
        "name": name,
        "email": email,
        "password": password,
    });

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_payload)
        .to_request();

    test::call_service(app, register_req).await
}

pub fn setup_test_env() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_testing");
    std::env::set_var("ACCESS_TOKEN_DURATION_MINUTES", "60");
    std::env::set_var("REFRESH_TOKEN_DURATION_DAYS", "30");
}
