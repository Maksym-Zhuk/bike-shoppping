use crate::{
    dto::auth::{AuthResponse, LoginDto, RefreshTokenRequest},
    errors::{AppErrors, auth_error::AuthError, hash_error::HashError, jwt_error::JWTError},
    models::role::Role,
    utils::{hash, jwt},
};
use actix_web::web;
use bson::doc;
use mongodb::Database;
use uuid::Uuid;

use crate::{
    dto::auth::{RegisterDto, UserInfo},
    models::user::User,
};

pub async fn register(
    db: &Database,
    data: web::Json<RegisterDto>,
) -> Result<AuthResponse, AppErrors> {
    let password_hash = match hash::hash_password(&data.password) {
        Ok(h) => h,
        Err(_) => {
            return Err(AppErrors::Hash(HashError::FailedHash));
        }
    };

    let user = User {
        _id: Uuid::new_v4(),
        email: data.email.clone(),
        name: data.name.clone(),
        password: password_hash,
        role: Role::User,
    };

    let collection = db.collection::<User>("users");

    collection.insert_one(&user).await?;

    let tokens = match jwt::generate_token_pair(user._id.to_string(), user.role) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("❌ Token generation error: {:#}", err);
            return Err(AppErrors::Jwt(JWTError::FailedGenerateTokens));
        }
    };

    Ok(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user: UserInfo {
            id: user._id.to_string(),
            email: user.email,
            name: user.name,
            role: user.role,
        },
    })
}

pub async fn login(db: &Database, data: web::Json<LoginDto>) -> Result<AuthResponse, AppErrors> {
    let collections = db.collection::<User>("users");

    let user = collections.find_one(doc! {"email": &data.email}).await?;

    match user {
        Some(user) => {
            let is_valid = match hash::verify_password(&data.password, &user.password) {
                Ok(valid) => valid,
                Err(err) => {
                    eprintln!("❌ Password verification error: {:#}", err);
                    return Err(AppErrors::Auth(AuthError::AuthFailed));
                }
            };

            if !is_valid {
                println!("❌ Invalid password for: {}", user.email);
                return Err(AppErrors::Auth(AuthError::InvalidEmailORPassword));
            }

            let tokens = match jwt::generate_token_pair(user._id.to_string(), user.role) {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("❌ Token generation error: {:#}", err);
                    return Err(AppErrors::Jwt(JWTError::FailedGenerateTokens));
                }
            };

            Ok(AuthResponse {
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
                user: UserInfo {
                    id: user._id.to_string(),
                    email: user.email,
                    name: user.name,
                    role: user.role,
                },
            })
        }
        None => {
            return Err(AppErrors::NotFound("User".to_string()));
        }
    }
}

pub async fn refresh_token(data: web::Json<RefreshTokenRequest>) -> Result<String, AppErrors> {
    let claims = match jwt::validate_token(data.refresh_token.clone()) {
        Ok(c) => c,
        Err(err) => {
            println!("❌ Invalid refresh token: {}", err);
            return Err(AppErrors::Jwt(JWTError::InvalidRefreshToken));
        }
    };

    let access_token = jwt::generate_access_token(claims.sub, claims.role)?;

    Ok(access_token)
}
