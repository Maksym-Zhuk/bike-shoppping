use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(TS, Serialize, Deserialize, Clone, ToSchema, Debug)]
#[ts(export, export_to = "../../db_types/Order.d.ts")]
pub struct Order {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub _id: Uuid,
    /// products id
    pub products_id: Vec<String>,
    pub total_price: u32,
    pub customer_id: String,
}
