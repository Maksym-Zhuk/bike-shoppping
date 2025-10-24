use crate::{
    dto::auth::{AuthResponse, LoginDto, RefreshTokenRequest, RefreshTokenResponse},
    utils::{hash, jwt},
};
use actix_web::web;
use anyhow::{Result, anyhow};
use bson::doc;
use mongodb::Database;
use uuid::Uuid;

use crate::{
    dto::auth::{RegisterDto, UserInfo},
    models::user::User,
};

pub async fn register(db: &Database, data: web::Json<RegisterDto>) -> Result<AuthResponse> {
    let password_hash = match hash::hash_password(&data.password) {
        Ok(h) => h,
        Err(_) => {
            return Err(anyhow!("Failed to process password"));
        }
    };

    let user = User {
        _id: Uuid::new_v4(),
        email: data.email.clone(),
        name: data.name.clone(),
        password: password_hash,
    };

    let collection = db.collection::<User>("users");

    collection.insert_one(&user).await?;

    let tokens = match jwt::generate_token_pair(user._id.to_string()) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("❌ Token generation error: {:#}", err);
            return Err(anyhow!("Failed to generate tokens"));
        }
    };

    Ok(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user: UserInfo {
            id: user._id.to_string(),
            email: user.email,
            name: user.name,
        },
    })
}

pub async fn login(db: &Database, data: web::Json<LoginDto>) -> Result<AuthResponse> {
    let collections = db.collection::<User>("users");

    let user = collections.find_one(doc! {"email": &data.email}).await?;

    match user {
        Some(user) => {
            let is_valid = match hash::verify_password(&data.password, &user.password) {
                Ok(valid) => valid,
                Err(err) => {
                    eprintln!("❌ Password verification error: {:#}", err);
                    return Err(anyhow!("Authentication failed"));
                }
            };

            if !is_valid {
                println!("❌ Invalid password for: {}", user.email);
                return Err(anyhow!("Invalid email or password"));
            }

            let tokens = match jwt::generate_token_pair(user._id.to_string()) {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("❌ Token generation error: {:#}", err);
                    return Err(anyhow!("Failed to generate tokens"));
                }
            };

            Ok(AuthResponse {
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
                user: UserInfo {
                    id: user._id.to_string(),
                    email: user.email,
                    name: user.name,
                },
            })
        }
        None => {
            return Err(anyhow!("Unable to find user"));
        }
    }
}

pub async fn refresh_token(data: web::Json<RefreshTokenRequest>) -> Result<RefreshTokenResponse> {
    let claims = match jwt::validate_token(data.refresh_token.clone()) {
        Ok(c) => c,
        Err(err) => {
            println!("❌ Invalid refresh token: {}", err);
            return Err(anyhow!("Invalid or expired refresh token"));
        }
    };

    let access_token = jwt::generate_access_token(claims.sub)?;

    Ok(RefreshTokenResponse { access_token })
}
