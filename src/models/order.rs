use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(TS, Serialize, Deserialize, Clone, ToSchema)]
#[ts(export)]
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
#[ts(export)]
pub struct CreateOrderDto {
    /// products id
    pub products_id: Vec<String>,
    pub total_price: u32,
}
