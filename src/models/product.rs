use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{ToSchema, openapi::schema};
use uuid::Uuid;

#[derive(TS, Serialize, Deserialize, Clone, ToSchema)]
#[ts(export)]
pub struct Product {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub _id: Uuid,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub images: Vec<String>,
    pub discount: u32,
    pub category: Category,
}

#[derive(TS, Serialize, Deserialize, Clone, ToSchema)]
#[ts(export)]
pub enum Category {
    helmet,
    bike,
}
