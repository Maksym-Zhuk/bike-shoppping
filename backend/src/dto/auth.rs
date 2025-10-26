use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use validator::Validate;

use crate::models::role::Role;

#[derive(TS, Serialize, Deserialize, Clone, ToSchema, Validate)]
#[ts(export, export_to = "../../db_types/LoginDto.d.ts")]
pub struct LoginDto {
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@example.com", format = "email")]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[schema(min_length = 8, example = "password123")]
    pub password: String,
}

#[derive(TS, Serialize, Deserialize, Clone, ToSchema, Validate)]
#[ts(export, export_to = "../../db_types/RegisterDto.d.ts")]
pub struct RegisterDto {
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@example.com", format = "email")]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[schema(min_length = 8, example = "password123")]
    pub password: String,
    #[validate(length(min = 2, max = 50))]
    #[schema(min_length = 2, max_length = 50, example = "John Doe")]
    pub name: String,
}

#[derive(TS, Serialize, Clone, ToSchema)]
#[ts(export, export_to = "../../db_types/AuthResponse.d.ts")]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserInfo,
}

#[derive(TS, Serialize, Clone, ToSchema)]
#[ts(export, export_to = "../../db_types/UserInfo.d.ts")]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: Role,
}

#[derive(TS, Deserialize, Clone, ToSchema)]
#[ts(export, export_to = "../../db_types/RefreshTokenRequest.d.ts")]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}
