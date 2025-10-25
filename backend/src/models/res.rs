use serde::Serialize;
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(TS, Serialize, Clone, ToSchema)]
#[ts(export, export_to = "../../db_types/MessageResponse.d.ts")]
pub struct MessageResponse {
    pub message: String,
}
