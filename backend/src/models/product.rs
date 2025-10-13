use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(TS, Serialize, Deserialize, Clone, ToSchema, Validate)]
#[ts(export, export_to = "../../db_types/Product.d.ts")]
pub struct Product {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub _id: Uuid,
    #[validate(length(min = 2, message = "The number of letters must be at least 2."))]
    pub name: String,
    pub price: u32,
    #[validate(length(min = 2, message = "The number of letters must be at least 2."))]
    pub description: String,
    pub images: Vec<String>,
    #[validate(range(
        min = 0,
        max = 100,
        message = "The discount must be between 0 and 100."
    ))]
    #[schema(example = 0, minimum = 0, maximum = 100)]
    pub discount: u8,
    /// 0: Helmet
    /// 1: Bike
    #[validate(range(min = 0, max = 1, message = "The category must be between 0 and 1."))]
    #[schema(example = 0, minimum = 0, maximum = 1)]
    pub category: u8,
}

#[derive(TS, Serialize, Deserialize, Clone, ToSchema, Validate)]
#[ts(export, export_to = "../../db_types/CreateProductDto.d.ts")]
pub struct CreateProductDto {
    #[validate(length(min = 2, message = "The number of letters must be at least 2."))]
    pub name: String,
    pub price: u32,
    #[validate(length(min = 2, message = "The number of letters must be at least 2."))]
    pub description: String,
    pub images: Vec<String>,
    #[validate(range(
        min = 0,
        max = 100,
        message = "The discount must be between 0 and 100."
    ))]
    #[schema(example = 0, minimum = 0, maximum = 100)]
    pub discount: u8,
    /// 0: Helmet
    /// 1: Bike
    #[validate(range(min = 0, max = 1, message = "The category must be between 0 and 1."))]
    #[schema(example = 0, minimum = 0, maximum = 1)]
    pub category: u8,
}

#[derive(TS, Serialize, Deserialize, Clone, ToSchema, Validate)]
#[ts(export, export_to = "../../db_types/UpdateProductDto.d.ts")]
pub struct UpdateProductDto {
    #[ts(type = "string")]
    #[schema(value_type = String)]
    pub _id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, message = "The number of letters must be at least 2."))]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, message = "The number of letters must be at least 2."))]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(
        min = 0,
        max = 100,
        message = "The discount must be between 0 and 100."
    ))]
    #[schema(example = 0, minimum = 0, maximum = 100)]
    pub discount: Option<u8>,
    /// 0: Helmet
    /// 1: Bike
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 1, message = "The category must be between 0 and 1."))]
    #[schema(example = 0, minimum = 0, maximum = 1)]
    pub category: Option<u8>,
}
