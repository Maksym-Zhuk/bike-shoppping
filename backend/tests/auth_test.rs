use actix_web::{http::StatusCode, test};
use bson::doc;
use serde_json::json;
mod common;

#[actix_web::test]
async fn test_register() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let payload = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "password": "SecurePass123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&payload)
        .to_request();

    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::CREATED);

    assert!(
        res.headers().get("x-access-token").is_some(),
        "access-token field missing in response"
    );
    assert!(
        res.headers().get("x-refresh-token").is_some(),
        "refresh-token field missing in response"
    );

    let body: serde_json::Value = test::read_body_json(res).await;

    println!("{:?}", body);

    assert_eq!(body["name"], "John Doe");
    assert_eq!(body["email"], "john@example.com");
    assert!(
        body.get("password").is_none(),
        "Password should not be returned"
    );

    let collection = db.collection::<mongodb::bson::Document>("users");
    let user = collection
        .find_one(doc! { "email": body["email"].as_str() })
        .await
        .unwrap();

    assert!(user.is_some(), "User should be in database");

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_register_duplicate_email() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis.clone()).await;

    let payload = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "password": "SecurePass123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&payload)
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&payload)
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), StatusCode::CONFLICT);

    let body: serde_json::Value = test::read_body_json(res).await;
    assert!(body["error"].as_str().unwrap().contains("duplicate_key"));

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_register_invalid_data() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis.clone()).await;

    let payload = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "password": "",
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&payload)
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::BAD_REQUEST);

    let body: serde_json::Value = test::read_body_json(res).await;
    assert!(body["error"].as_str().unwrap().contains("validation_error"));

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_login() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_payload = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "password": "SecurePass123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_res = test::call_service(&app, register_req).await;

    assert_eq!(register_res.status(), StatusCode::CREATED);

    let payload = json!({
        "email": "john@example.com",
        "password": "SecurePass123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&payload)
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    assert!(
        res.headers().get("x-access-token").is_some(),
        "access-token field missing in response"
    );
    assert!(
        res.headers().get("x-refresh-token").is_some(),
        "refresh-token field missing in response"
    );

    let register_body: serde_json::Value = test::read_body_json(register_res).await;
    let body: serde_json::Value = test::read_body_json(res).await;

    assert_eq!(body["id"], register_body["id"]);
    assert_eq!(body["name"], "John Doe");
    assert_eq!(body["email"], "john@example.com");
    assert_eq!(body["role"], "User");

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_login_invalid_data() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_payload = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "password": "SecurePass123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_res = test::call_service(&app, register_req).await;

    assert_eq!(register_res.status(), StatusCode::CREATED);

    let payload = json!({
        "email": "john@example.com",
        "password": "SecurePass!",
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&payload)
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let body: serde_json::Value = test::read_body_json(res).await;
    assert!(body["error"]
        .as_str()
        .unwrap()
        .contains("invalid_credentials"));

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_refresh_token() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_payload = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "password": "SecurePass123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_res = test::call_service(&app, register_req).await;

    assert_eq!(register_res.status(), StatusCode::CREATED);

    let refresh_token = register_res
        .headers()
        .get("x-refresh-token")
        .expect("Refresh token header missing")
        .to_str()
        .expect("Refresh token header is not valid UTF-8");

    let req = test::TestRequest::post()
        .uri("/api/auth/refresh_token")
        .insert_header(("X-Refresh-Token", refresh_token))
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    assert!(
        res.headers().get("x-access-token").is_some(),
        "access-token field missing in response"
    );

    let body: serde_json::Value = test::read_body_json(res).await;

    assert_eq!(body["message"], "Token refreshed successfully");

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_refresh_token_invalid_token() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_payload = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "password": "SecurePass123!",
    });

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_payload)
        .to_request();
    let register_res = test::call_service(&app, register_req).await;

    assert_eq!(register_res.status(), StatusCode::CREATED);

    let mut refresh_token = register_res
        .headers()
        .get("x-refresh-token")
        .expect("Refresh token header missing")
        .to_str()
        .expect("Refresh token header is not valid UTF-8")
        .to_string();

    refresh_token.pop();

    let req = test::TestRequest::post()
        .uri("/api/auth/refresh_token")
        .insert_header(("X-Refresh-Token", refresh_token))
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert!(body["error"]
        .as_str()
        .unwrap()
        .contains("invalid_refresh_token"));

    common::teardown_test_db(&db).await;
}
