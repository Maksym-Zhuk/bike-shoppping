use actix_web::web;
use bson::to_document;
use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Database, options::FindOneOptions};
use redis::{AsyncCommands, aio::ConnectionManager};
use uuid::Uuid;

use crate::models::product::{CreateProductDto, Product, UpdateProductDto};

pub async fn get_all_products(db: &Database) -> mongodb::error::Result<Vec<Product>> {
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
) -> mongodb::error::Result<String> {
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

pub async fn get_product(
    db: &Database,
    product_id: &str,
) -> mongodb::error::Result<Option<Product>> {
    let collection = db.collection::<Product>("products");

    let uuid = Uuid::parse_str(product_id)
        .map_err(|e| mongodb::error::Error::custom(format!("Invalid UUID: {}", e)))?;

    let product = collection.find_one(doc! {"_id": uuid}).await?;
    Ok(product)
}

pub async fn update_product(
    db: &Database,
    mut redis: ConnectionManager,
    new_product_data: web::Json<UpdateProductDto>,
) -> mongodb::error::Result<Option<String>> {
    let collection = db.collection::<Product>("products");

    let uuid = Uuid::parse_str(&new_product_data._id)
        .map_err(|e| mongodb::error::Error::custom(format!("Invalid UUID: {}", e)))?;

    let mut update_doc = to_document(&new_product_data)?;
    update_doc.remove("_id");

    collection
        .update_one(doc! {"_id": uuid}, doc! {"$set": update_doc})
        .await?;

    let _: std::result::Result<(), _> = redis.del("most_advantageous").await;

    Ok(Some(String::from("Product updated successfully")))
}

pub async fn delete_product(
    db: &Database,
    mut redis: ConnectionManager,
    product_id: &str,
) -> mongodb::error::Result<Option<String>> {
    let collection = db.collection::<Product>("products");

    let uuid = Uuid::parse_str(product_id)
        .map_err(|e| mongodb::error::Error::custom(format!("Invalid UUID: {}", e)))?;

    collection.delete_one(doc! {"_id": uuid}).await?;

    let _: std::result::Result<(), _> = redis.del("most_advantageous").await;

    Ok(Some(String::from("Product deleted successfully")))
}

pub async fn get_most_advantageous(
    db: &Database,
    mut redis: ConnectionManager,
) -> mongodb::error::Result<Option<Product>> {
    if let Ok(cached_json) = redis.get::<_, String>("most_advantageous").await {
        if let Ok(best_product) = serde_json::from_str::<Product>(&cached_json) {
            return Ok(Some(best_product));
        }
    }

    let collection = db.collection::<Product>("products");

    let options = FindOneOptions::builder()
        .sort(doc! { "discount": -1 })
        .build();

    let best_product = collection.find_one(doc! {}).with_options(options).await?;

    if let Some(ref product) = best_product {
        if let Ok(json) = serde_json::to_string(product) {
            let _: std::result::Result<(), _> = redis.set_ex("most_advantageous", json, 3600).await;
        }
    }

    Ok(best_product)
}
