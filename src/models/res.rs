use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    #[schema(example = "Internal server error")]
    pub message: String,
}
