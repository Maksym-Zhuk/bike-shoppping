use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(TS, Serialize, Deserialize, Clone, ToSchema, Validate, Debug)]
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
