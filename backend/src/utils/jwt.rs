use anyhow::Ok;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

impl Claims {
    pub fn new(user_id: String, duration_minutes: i64) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::minutes(duration_minutes)).timestamp();

        Self {
            sub: user_id,
            exp,
            iat: now.timestamp(),
        }
    }
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "development-secret-key-change-in-production".to_string())
}

pub fn generate_access_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, ACCESS_TOKEN_DURATION_MINUTES);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_ref()),
    )
}

pub fn generate_refresh_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, REFRESH_TOKEN_DURATION_DAYS * 24 * 60);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_ref()),
    )
}

pub fn generate_token_pair(user_id: String) -> Result<TokenPair, jsonwebtoken::errors::Error> {
    let access_token = generate_access_token(user_id.clone());
    let refresh_token = generate_refresh_token(user_id);

    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}

pub fn validate_token(token: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_ref()),
        &validation,
    );

    Ok(token_data.claims)
}
