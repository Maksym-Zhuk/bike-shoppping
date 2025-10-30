use actix_web::web;
use bson::to_document;
use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{options::FindOneOptions, Database};
use redis::{aio::ConnectionManager, AsyncCommands};
use uuid::Uuid;

use crate::dto::product::{CreateProductDto, UpdateProductDto};
use crate::errors::AppErrors;
use crate::models::product::Product;

pub async fn get_all_products(db: &Database) -> Result<Vec<Product>, AppErrors> {
    let collection = db.collection::<Product>("products");

    let mut cursor = collection.find(doc! {}).await?;
    let mut products = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        products.push(result);
    }

    Ok(products)
}

pub async fn create_product(
    db: &Database,
    mut redis: ConnectionManager,
    new_product_data: web::Json<CreateProductDto>,
) -> Result<String, AppErrors> {
    let new_product = Product {
        _id: Uuid::new_v4(),
        name: new_product_data.name.to_string(),
        category: new_product_data.category,
        description: new_product_data.description.to_string(),
        discount: new_product_data.discount,
        images: new_product_data.images.clone(),
        price: new_product_data.price,
    };

    let collection = db.collection::<Product>("products");
    collection.insert_one(&new_product).await?;

    let _: std::result::Result<(), _> = redis.del("most_advantageous").await;

    Ok(String::from("Product created successfully"))
}

pub async fn get_product(db: &Database, product_id: &str) -> Result<Product, AppErrors> {
    let collection = db.collection::<Product>("products");

    let uuid = Uuid::parse_str(product_id)
        .map_err(|e| mongodb::error::Error::custom(format!("Invalid UUID: {}", e)))?;

    let product = collection
        .find_one(doc! {"_id": uuid})
        .await?
        .ok_or_else(|| AppErrors::NotFound(product_id.to_string()))?;
    Ok(product)
}

pub async fn update_product(
    db: &Database,
    mut redis: ConnectionManager,
    new_product_data: web::Json<UpdateProductDto>,
) -> Result<String, AppErrors> {
    let collection = db.collection::<Product>("products");

    let uuid = Uuid::parse_str(&new_product_data._id)
        .map_err(|e| mongodb::error::Error::custom(format!("Invalid UUID: {}", e)))?;

    let mut update_doc = to_document(&new_product_data)?;
    update_doc.remove("_id");

    collection
        .update_one(doc! {"_id": uuid}, doc! {"$set": update_doc})
        .await?;

    let _: std::result::Result<(), _> = redis.del("most_advantageous").await;

    Ok(String::from("Product updated successfully"))
}

pub async fn delete_product(
    db: &Database,
    mut redis: ConnectionManager,
    product_id: &str,
) -> Result<String, AppErrors> {
    let collection = db.collection::<Product>("products");

    let uuid = Uuid::parse_str(product_id).map_err(|_| AppErrors::InvalidUUID)?;

    collection.delete_one(doc! {"_id": uuid}).await?;

    let _: std::result::Result<(), _> = redis.del("most_advantageous").await;

    Ok(String::from("Product deleted successfully"))
}

pub async fn get_most_advantageous(
    db: &Database,
    mut redis: ConnectionManager,
) -> Result<Product, AppErrors> {
    if let Ok(cached_json) = redis.get::<_, String>("most_advantageous").await {
        if let Ok(best_product) = serde_json::from_str::<Product>(&cached_json) {
            return Ok(best_product);
        }
    }

    let collection = db.collection::<Product>("products");

    let options = FindOneOptions::builder()
        .sort(doc! { "discount": -1 })
        .build();

    let best_product = collection
        .find_one(doc! {})
        .with_options(options)
        .await?
        .ok_or(AppErrors::NotFound("Product".to_string()))?;

    if let Ok(json) = serde_json::to_string(&best_product) {
        let _: std::result::Result<(), _> = redis.set_ex("most_advantageous", json, 3600).await;
    }

    Ok(best_product)
}
