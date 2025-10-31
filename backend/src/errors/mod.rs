pub mod auth_error;
pub mod hash_error;
pub mod jwt_error;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Debug, Error)]
pub enum AppErrors {
    #[error("Database error")]
    Mongo(#[from] mongodb::error::Error),

    #[error("BSON serialization error: {0}")]
    Bson(#[from] bson::ser::Error),

    #[error(transparent)]
    Jwt(#[from] jwt_error::JWTError),

    #[error(transparent)]
    Hash(#[from] hash_error::HashError),

    #[error(transparent)]
    Auth(#[from] auth_error::AuthError),

    #[error("Invalid UUID")]
    InvalidUUID,

    #[error("{0} not found")]
    NotFound(String),
}

#[derive(TS, Serialize, Clone, ToSchema)]
#[ts(export, export_to = "../../db_types/ErrorResponse.d.ts")]
pub struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl ResponseError for AppErrors {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type, message, details) = match self {
            AppErrors::Jwt(e) => match e {
                jwt_error::JWTError::FailedDecode => (
                    StatusCode::UNAUTHORIZED,
                    "failed_decode",
                    "Unable to decode token".to_string(),
                    None,
                ),
                jwt_error::JWTError::FailedGenerateAccessToken => (
                    StatusCode::UNAUTHORIZED,
                    "failed_generate",
                    "Failed generate access token".to_string(),
                    None,
                ),
                jwt_error::JWTError::FailedGenerateRefreshToken => (
                    StatusCode::UNAUTHORIZED,
                    "failed_generate",
                    "Failed generate refresh token".to_string(),
                    None,
                ),
                jwt_error::JWTError::FailedGenerateTokens => (
                    StatusCode::UNAUTHORIZED,
                    "failed_generate",
                    "Failed generate tokens".to_string(),
                    None,
                ),
                jwt_error::JWTError::InvalidRefreshToken => (
                    StatusCode::UNAUTHORIZED,
                    "invalid_refresh_token",
                    "Invalid refresh token".to_string(),
                    None,
                )
            },

            AppErrors::Auth(e) => match e {
                auth_error::AuthError::AuthFailed => (
                    StatusCode::UNAUTHORIZED,
                    "auth_failed",
                    "Auth failed".to_string(),
                    None,
                ),
                auth_error::AuthError::InvalidEmailORPassword => (
                    StatusCode::UNAUTHORIZED, 
                    "invalid_credentials",
                    "Invalid email or password".to_string(),
                    None,
                ),
                auth_error::AuthError::Unauthorized => (
                    StatusCode::UNAUTHORIZED,
                    "unauthorized",
                    "No claims found".to_string(),
                    None
                )
            },

            AppErrors::Hash(_e) => {
                eprintln!("Hash error: {:?}", _e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "hash_error",
                    "Password processing error".to_string(),
                    None,
                )
            }

            AppErrors::Mongo(e) => {
                use mongodb::error::{ErrorKind, WriteFailure};

                eprintln!("MongoDB error: {:?}", e);

                match &*e.kind {
                    ErrorKind::Write(WriteFailure::WriteError(write_err)) if write_err.code == 11000 => (
                        StatusCode::CONFLICT,
                        "duplicate_key",
                        "Email already exists".to_string(),
                        None,
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "database_error",
                        "Database error".to_string(),
                        Some(e.to_string()),
                    ),
                }
            },

            AppErrors::Bson(e) => {
                eprintln!("BSON error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "serialization_error",
                    "Data serialization error".to_string(),
                    None,
                )
            }

            AppErrors::InvalidUUID => (
                StatusCode::BAD_REQUEST,
                "invalid_uuid",
                "Invalid UUID format".to_string(),
                Some("The UUID must be in the following format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string()),
            ),
            
            AppErrors::NotFound(resource) => (
                StatusCode::NOT_FOUND,
                "not_found",
                format!("{} not found", resource),  
                None,
            ),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message,
            details,
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppErrors::Jwt(_) => StatusCode::UNAUTHORIZED,
            AppErrors::Auth(_) => StatusCode::UNAUTHORIZED,
            AppErrors::Hash(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrors::Mongo(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrors::Bson(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrors::InvalidUUID => StatusCode::BAD_REQUEST,
            AppErrors::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}