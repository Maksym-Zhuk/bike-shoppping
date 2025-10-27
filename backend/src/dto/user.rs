use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use validator::Validate;

#[derive(TS, Deserialize, Clone, ToSchema, Validate, Serialize)]
#[ts(export, export_to = "../../db_types/UpdateUserDto.d.ts")]
pub struct UpdateUserDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, max = 50))]
    #[schema(min_length = 2, max_length = 50, example = "John Doe")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@example.com", format = "email")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[schema(min_length = 8, example = "password123")]
    password: Option<String>,
}
