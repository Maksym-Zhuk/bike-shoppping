use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(TS, Serialize, Deserialize, Clone, ToSchema)]
#[ts(export, export_to = "../../db_types/Order.d.ts")]
pub struct Order {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub _id: Uuid,
    /// products id
    pub products_id: Vec<String>,
    pub total_price: u32,
}

#[derive(TS, Serialize, Deserialize, Clone, ToSchema)]
#[ts(export, export_to = "../../db_types/CreateOrderDto.d.ts")]
pub struct CreateOrderDto {
    /// products id
    pub products_id: Vec<String>,
    pub total_price: u32,
}

#[derive(TS, Serialize, Deserialize, Clone, ToSchema)]
#[ts(export, export_to = "../../db_types/UpdateOrderDto.d.ts")]
pub struct UpdateOrderDto {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub _id: String,
    /// products id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<u32>,
}
