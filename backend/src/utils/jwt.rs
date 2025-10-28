use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::{
    errors::{jwt_error::JWTError, AppErrors},
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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_claims_new_creates_valid_claims() {
        let claims = Claims::new("user123".to_string(), 60, Role::User);

        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, Role::User);
        assert!(claims.exp > claims.iat, "exp should be after iat");

        let duration = claims.exp - claims.iat;
        assert!(
            duration >= 59 * 60 && duration <= 61 * 60,
            "Duration should be ~60 minutes"
        );
    }

    #[test]
    fn test_claims_expiration_calculation() {
        let claims = Claims::new("user123".to_string(), 120, Role::Admin);

        let duration_minutes = (claims.exp - claims.iat) / 60;
        assert!(
            duration_minutes >= 119 && duration_minutes <= 121,
            "Should expire in ~120 minutes"
        );
    }

    #[test]
    #[serial]
    fn test_config_defaults() {
        env::remove_var("JWT_SECRET");
        env::remove_var("ACCESS_TOKEN_DURATION_MINUTES");
        env::remove_var("REFRESH_TOKEN_DURATION_DAYS");

        assert_eq!(
            get_jwt_secret(),
            "development-secret-key-change-in-production"
        );
        assert_eq!(get_access_token_duration(), 60);
        assert_eq!(get_refresh_token_duration(), 30);
    }

    #[test]
    #[serial]
    fn test_config_from_env() {
        env::set_var("JWT_SECRET", "test-secret");
        env::set_var("ACCESS_TOKEN_DURATION_MINUTES", "120");
        env::set_var("REFRESH_TOKEN_DURATION_DAYS", "7");

        assert_eq!(get_jwt_secret(), "test-secret");
        assert_eq!(get_access_token_duration(), 120);
        assert_eq!(get_refresh_token_duration(), 7);

        env::remove_var("JWT_SECRET");
        env::remove_var("ACCESS_TOKEN_DURATION_MINUTES");
        env::remove_var("REFRESH_TOKEN_DURATION_DAYS");
    }

    #[test]
    #[serial]
    fn test_config_invalid_numbers_use_defaults() {
        env::set_var("ACCESS_TOKEN_DURATION_MINUTES", "invalid");
        env::set_var("REFRESH_TOKEN_DURATION_DAYS", "not_a_number");

        assert_eq!(get_access_token_duration(), 60);
        assert_eq!(get_refresh_token_duration(), 30);

        env::remove_var("ACCESS_TOKEN_DURATION_MINUTES");
        env::remove_var("REFRESH_TOKEN_DURATION_DAYS");
    }

    #[test]
    #[serial]
    fn test_generate_access_token_success() {
        env::set_var("JWT_SECRET", "test-secret-key");
        env::set_var("ACCESS_TOKEN_DURATION_MINUTES", "60");

        let result = generate_access_token("user123".to_string(), Role::User);

        assert!(result.is_ok(), "Should generate access token");

        let token = result.unwrap();
        assert!(!token.is_empty(), "Token should not be empty");
        assert!(token.contains('.'), "JWT should have dots");

        env::remove_var("JWT_SECRET");
        env::remove_var("ACCESS_TOKEN_DURATION_MINUTES");
    }

    #[test]
    #[serial]
    fn test_generate_access_token_different_roles() {
        env::set_var("JWT_SECRET", "test-secret");

        let user_token = generate_access_token("user1".to_string(), Role::User).unwrap();
        let admin_token = generate_access_token("admin1".to_string(), Role::Admin).unwrap();

        assert_ne!(
            user_token, admin_token,
            "Different users should have different tokens"
        );

        env::remove_var("JWT_SECRET");
    }

    #[test]
    #[serial]
    fn test_generate_refresh_token_success() {
        env::set_var("JWT_SECRET", "test-secret-key");
        env::set_var("REFRESH_TOKEN_DURATION_DAYS", "30");

        let result = generate_refresh_token("user123".to_string(), Role::User);

        assert!(result.is_ok(), "Should generate refresh token");

        let token = result.unwrap();
        assert!(!token.is_empty(), "Token should not be empty");
        assert!(token.contains('.'), "JWT should have dots");

        env::remove_var("JWT_SECRET");
        env::remove_var("REFRESH_TOKEN_DURATION_DAYS");
    }

    #[test]
    #[serial]
    fn test_generate_token_pair_success() {
        env::set_var("JWT_SECRET", "test-secret-key");
        env::set_var("ACCESS_TOKEN_DURATION_MINUTES", "60");
        env::set_var("REFRESH_TOKEN_DURATION_DAYS", "30");

        let result = generate_token_pair("user123".to_string(), Role::User);

        assert!(result.is_ok(), "Should generate token pair");

        let pair = result.unwrap();
        assert!(
            !pair.access_token.is_empty(),
            "Access token should not be empty"
        );
        assert!(
            !pair.refresh_token.is_empty(),
            "Refresh token should not be empty"
        );
        assert_ne!(
            pair.access_token, pair.refresh_token,
            "Tokens should be different"
        );

        env::remove_var("JWT_SECRET");
        env::remove_var("ACCESS_TOKEN_DURATION_MINUTES");
        env::remove_var("REFRESH_TOKEN_DURATION_DAYS");
    }

    #[test]
    #[serial]
    fn test_validate_token_success() {
        env::set_var("JWT_SECRET", "test-secret-key");
        env::set_var("ACCESS_TOKEN_DURATION_MINUTES", "60");

        let token = generate_access_token("user123".to_string(), Role::User).unwrap();
        let result = validate_token(token);

        assert!(result.is_ok(), "Should validate valid token");

        let claims = result.unwrap();
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, Role::User);
        assert!(claims.exp > claims.iat);

        env::remove_var("JWT_SECRET");
        env::remove_var("ACCESS_TOKEN_DURATION_MINUTES");
    }

    #[test]
    #[serial]
    fn test_validate_token_invalid() {
        env::set_var("JWT_SECRET", "test-secret-key");

        let result = validate_token("invalid.token.string".to_string());

        assert!(result.is_err(), "Should fail for invalid token");
        assert!(matches!(
            result.unwrap_err(),
            AppErrors::Jwt(JWTError::FailedDecode)
        ));

        env::remove_var("JWT_SECRET");
    }

    #[test]
    #[serial]
    fn test_validate_token_wrong_secret() {
        env::set_var("JWT_SECRET", "secret1");
        let token = generate_access_token("user123".to_string(), Role::User).unwrap();

        env::set_var("JWT_SECRET", "secret2");
        let result = validate_token(token);

        assert!(result.is_err(), "Should fail with wrong secret");

        env::remove_var("JWT_SECRET");
    }

    #[test]
    #[serial]
    fn test_validate_token_expired() {
        env::set_var("JWT_SECRET", "test-secret");

        let claims = Claims::new("user123".to_string(), -10, Role::User);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("test-secret".as_ref()),
        )
        .unwrap();

        let result = validate_token(token);

        assert!(result.is_err(), "Should fail for expired token");

        env::remove_var("JWT_SECRET");
    }
}
