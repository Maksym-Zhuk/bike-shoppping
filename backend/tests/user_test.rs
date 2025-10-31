use actix_web::{http::StatusCode, test};
use bike_shopping_backend::models::user::User;
use bson::doc;
use serde_json::json;
use uuid::Uuid;

mod common;

#[actix_web::test]
async fn test_get_me() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_res =
        common::register_test_user(&app, "John Doe", "john@example.com", "SecurePass123!").await;
    assert_eq!(register_res.status(), StatusCode::CREATED);

    let access_token = register_res
        .headers()
        .get("x-access-token")
        .expect("Access token header missing")
        .to_str()
        .expect("Access token header is not valid UTF-8")
        .to_string();

    let req = test::TestRequest::get()
        .uri("/api/user/me")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    let register_body: serde_json::Value = test::read_body_json(register_res).await;
    let body: serde_json::Value = test::read_body_json(res).await;

    assert_eq!(body["id"], register_body["id"]);
    assert_eq!(body["name"], "John Doe");
    assert_eq!(body["email"], "john@example.com");
    assert_eq!(body["role"], "User");

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_get_me_invalid_token() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_res =
        common::register_test_user(&app, "John Doe", "john@example.com", "SecurePass123!").await;
    assert_eq!(register_res.status(), StatusCode::CREATED);

    let mut access_token = register_res
        .headers()
        .get("x-access-token")
        .expect("Access token header missing")
        .to_str()
        .expect("Access token header is not valid UTF-8")
        .to_string();

    access_token.pop();

    let req = test::TestRequest::get()
        .uri("/api/user/me")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert!(body["error"].as_str().unwrap().contains("invalid_token"));

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_update_user() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_res =
        common::register_test_user(&app, "John Doe", "john@example.com", "SecurePass123!").await;
    assert_eq!(register_res.status(), StatusCode::CREATED);

    let access_token = register_res
        .headers()
        .get("x-access-token")
        .expect("Access token header missing")
        .to_str()
        .expect("Access token header is not valid UTF-8")
        .to_string();

    let payload = json!({
        "name": "Maksym"
    });

    let req = test::TestRequest::put()
        .uri("/api/user/update")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .set_json(&payload)
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert_eq!(
        body["message"].as_str().unwrap(),
        String::from("User updated successfully")
    );

    let register_body: serde_json::Value = test::read_body_json(register_res).await;

    let collection = db.collection::<User>("users");

    let uuid = Uuid::parse_str(register_body["id"].as_str().unwrap()).unwrap();

    let user = collection.find_one(doc! { "_id": uuid  }).await.unwrap();

    assert!(user.is_some(), "User should exist in database");
    assert_eq!(user.unwrap().name, "Maksym");

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_update_user_invalid_data() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_res =
        common::register_test_user(&app, "John Doe", "john@example.com", "SecurePass123!").await;
    assert_eq!(register_res.status(), StatusCode::CREATED);

    let access_token = register_res
        .headers()
        .get("x-access-token")
        .expect("Access token header missing")
        .to_str()
        .expect("Access token header is not valid UTF-8")
        .to_string();

    let payload = json!({
        "name": "M"
    });

    let req = test::TestRequest::put()
        .uri("/api/user/update")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
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
async fn test_update_user_invalid_token() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_res =
        common::register_test_user(&app, "John Doe", "john@example.com", "SecurePass123!").await;
    assert_eq!(register_res.status(), StatusCode::CREATED);

    let mut access_token = register_res
        .headers()
        .get("x-access-token")
        .expect("Access token header missing")
        .to_str()
        .expect("Access token header is not valid UTF-8")
        .to_string();

    access_token.pop();

    let payload = json!({
        "name": "Maksym"
    });

    let req = test::TestRequest::put()
        .uri("/api/user/update")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .set_json(&payload)
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert!(body["error"].as_str().unwrap().contains("invalid_token"));

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_delete_user() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_res =
        common::register_test_user(&app, "John Doe", "john@example.com", "SecurePass123!").await;
    assert_eq!(register_res.status(), StatusCode::CREATED);

    let access_token = register_res
        .headers()
        .get("x-access-token")
        .expect("Access token header missing")
        .to_str()
        .expect("Access token header is not valid UTF-8")
        .to_string();

    let req = test::TestRequest::delete()
        .uri("/api/user/delete")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert_eq!(
        body["message"].as_str().unwrap(),
        String::from("User deleted successfully")
    );

    let register_body: serde_json::Value = test::read_body_json(register_res).await;

    let collection = db.collection::<User>("users");

    let uuid = Uuid::parse_str(register_body["id"].as_str().unwrap()).unwrap();

    let user = collection.find_one(doc! { "_id": uuid  }).await.unwrap();

    assert!(
        user.is_none(),
        "User should have been deleted from database"
    );

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_delete_user_not_found_user() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_res =
        common::register_test_user(&app, "John Doe", "john@example.com", "SecurePass123!").await;
    assert_eq!(register_res.status(), StatusCode::CREATED);

    let access_token = register_res
        .headers()
        .get("x-access-token")
        .expect("Access token header missing")
        .to_str()
        .expect("Access token header is not valid UTF-8")
        .to_string();

    let req = test::TestRequest::delete()
        .uri("/api/user/delete")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert_eq!(
        body["message"].as_str().unwrap(),
        String::from("User deleted successfully")
    );

    let req = test::TestRequest::delete()
        .uri("/api/user/delete")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    let body: serde_json::Value = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);

    assert!(body["error"].as_str().unwrap().contains("not_found"));

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_delete_user_invalid_token() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_res =
        common::register_test_user(&app, "John Doe", "john@example.com", "SecurePass123!").await;
    assert_eq!(register_res.status(), StatusCode::CREATED);

    let mut access_token = register_res
        .headers()
        .get("x-access-token")
        .expect("Access token header missing")
        .to_str()
        .expect("Access token header is not valid UTF-8")
        .to_string();

    access_token.pop();

    let req = test::TestRequest::delete()
        .uri("/api/user/delete")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    let status = res.status();

    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert!(body["error"].as_str().unwrap().contains("invalid_token"));

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_get_users() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let admin_token = common::generate_test_admin_token().await.unwrap();

    let req = test::TestRequest::get()
        .uri("/api/user/admin/users")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    println!("{:?}", res);

    let status = res.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert_eq!(body, json!([]));

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_get_users_invalid_token() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let mut admin_token = common::generate_test_admin_token().await.unwrap();

    admin_token.pop();

    let req = test::TestRequest::get()
        .uri("/api/user/admin/users")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    println!("{:?}", res);

    let status = res.status();

    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert!(body["error"].as_str().unwrap().contains("invalid_token"));

    common::teardown_test_db(&db).await;
}

#[actix_web::test]
async fn test_get_users_no_admin_token() {
    let db = common::setup_test_db().await;
    let redis = common::setup_test_redis().await;
    let app = common::create_test_app(db.clone(), redis).await;

    let register_res =
        common::register_test_user(&app, "John Doe", "john@example.com", "SecurePass123!").await;
    assert_eq!(register_res.status(), StatusCode::CREATED);

    let access_token = register_res
        .headers()
        .get("x-access-token")
        .expect("Access token header missing")
        .to_str()
        .expect("Access token header is not valid UTF-8")
        .to_string();

    let req = test::TestRequest::get()
        .uri("/api/user/admin/users")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();
    let res = test::call_service(&app, req).await;

    println!("{:?}", res);

    let status = res.status();

    assert_eq!(status, StatusCode::FORBIDDEN);

    let body: serde_json::Value = test::read_body_json(res).await;

    assert!(body["error"]
        .as_str()
        .unwrap()
        .contains("insufficient_permissions"));

    common::teardown_test_db(&db).await;
}
