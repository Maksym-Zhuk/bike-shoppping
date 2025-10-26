use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

use crate::{
    errors::{AppErrors, jwt_error::JWTError},
    models::role::Role,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

impl Claims {
    pub fn new(user_id: String, duration_minutes: i64, role: Role) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::minutes(duration_minutes)).timestamp();

        Self {
            sub: user_id,
            role,
            exp,
            iat: now.timestamp(),
        }
    }
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "development-secret-key-change-in-production".to_string())
}

fn get_access_token_duration() -> i64 {
    env::var("ACCESS_TOKEN_DURATION_MINUTES")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(60)
}

fn get_refresh_token_duration() -> i64 {
    env::var("REFRESH_TOKEN_DURATION_DAYS")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(30)
}

pub fn generate_access_token(user_id: String, role: Role) -> Result<String, AppErrors> {
    let duration = get_access_token_duration();
    let claims = Claims::new(user_id, duration, role);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_ref()),
    )
    .map_err(|_| AppErrors::Jwt(JWTError::FailedGenerateAccessToken))
}

pub fn generate_refresh_token(user_id: String, role: Role) -> Result<String, AppErrors> {
    let duration = get_refresh_token_duration();
    let claims = Claims::new(user_id, duration * 24 * 60, role);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_ref()),
    )
    .map_err(|_| AppErrors::Jwt(JWTError::FailedGenerateRefreshToken))
}

pub fn generate_token_pair(user_id: String, role: Role) -> Result<TokenPair, AppErrors> {
    let access_token = generate_access_token(user_id.clone(), role.clone())
        .map_err(|_| AppErrors::Jwt(JWTError::FailedGenerateAccessToken))?;
    let refresh_token = generate_refresh_token(user_id, role)
        .map_err(|_| AppErrors::Jwt(JWTError::FailedGenerateRefreshToken))?;

    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}

pub fn validate_token(token: String) -> Result<Claims, AppErrors> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_ref()),
        &validation,
    )
    .map_err(|_| AppErrors::Jwt(JWTError::FailedDecode))?;

    Ok(token_data.claims)
}
