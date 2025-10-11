use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::openapi::schema::{Schema, SchemaType};
use utoipa::openapi::{ObjectBuilder, schema};
use utoipa::{ToSchema, openapi::example};
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
    #[schema(example = 0, minimum = 0, maximum = 100)]
    pub discount: u8,
    /// 0: Helmet
    /// 1: Bike
    #[schema(example = 0, minimum = 0, maximum = 1)]
    pub category: u8,
}

#[derive(TS, Serialize, Deserialize, Clone, ToSchema)]
#[ts(export)]
pub struct CreateProductDto {
    pub name: String,
    pub price: u32,
    pub description: String,
    pub images: Vec<String>,
    #[schema(example = 0, minimum = 0, maximum = 100)]
    pub discount: u8,
    /// 0: Helmet
    /// 1: Bike
    #[schema(example = 0, minimum = 0, maximum = 1)]
    pub category: u8,
}
