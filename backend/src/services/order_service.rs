use actix_web::web;
use anyhow::{Result, anyhow};
use bson::{Binary, Bson, to_document};
use futures_util::stream::TryStreamExt;
use mongodb::Database;
use mongodb::bson::doc;
use uuid::Uuid;

use crate::{
    dto::order::{CreateOrderDto, UpdateOrderDto},
    models::{order::Order, product::Product},
};

pub async fn get_all_orders(db: &Database) -> mongodb::error::Result<Vec<Order>> {
    let collection = db.collection::<Order>("orders");
    let mut cursor = collection.find(doc! {}).await?;

    let mut orders = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        orders.push(result);
    }

    Ok(orders)
}

pub async fn create_order(
    db: &Database,
    new_order_data: web::Json<CreateOrderDto>,
) -> Result<String> {
    let orders_collection = db.collection::<Order>("orders");
    let products_collection = db.collection::<Product>("products");

    let uuids: Vec<Bson> = new_order_data
        .products_id
        .iter()
        .filter_map(|id| Uuid::parse_str(id).ok())
        .map(|uuid| {
            Bson::Binary(Binary {
                subtype: bson::spec::BinarySubtype::Uuid,
                bytes: uuid.as_bytes().to_vec(),
            })
        })
        .collect();

    let mut cursor = products_collection
        .find(doc! { "_id": {"$in": uuids}})
        .await?;

    let mut products = Vec::new();
    while let Some(p) = cursor.try_next().await? {
        products.push(p);
    }

    if products.len() != new_order_data.products_id.len() {
        return Err(anyhow!("Some products not found"));
    }

    let order = Order {
        _id: Uuid::new_v4(),
        products_id: new_order_data.products_id.clone(),
        total_price: new_order_data.total_price,
    };

    orders_collection.insert_one(order).await?;

    Ok(String::from("Order created successfully"))
}

pub async fn get_order(db: &Database, order_id: &str) -> mongodb::error::Result<Option<Order>> {
    let collection = db.collection::<Order>("orders");

    let uuid = Uuid::parse_str(order_id)
        .map_err(|e| mongodb::error::Error::custom(format!("Invalid UUID: {}", e)))?;

    let order = collection.find_one(doc! {"_id": uuid}).await?;
    Ok(order)
}

pub async fn update_order(
    db: &Database,
    new_order_data: web::Json<UpdateOrderDto>,
) -> mongodb::error::Result<Option<String>> {
    let collection = db.collection::<Order>("orders");

    let uuid = Uuid::parse_str(&new_order_data._id)
        .map_err(|e| mongodb::error::Error::custom(format!("Invalid UUID: {}", e)))?;

    let mut update_doc = to_document(&new_order_data)?;

    update_doc.remove("_id");

    collection
        .update_one(doc! {"_id": uuid}, doc! {"$set": update_doc})
        .await?;

    Ok(Some(String::from("Product updated successfully")))
}

pub async fn delete_order(db: &Database, order_id: &str) -> mongodb::error::Result<Option<String>> {
    let collection = db.collection::<Order>("orders");

    let uuid = Uuid::parse_str(order_id)
        .map_err(|e| mongodb::error::Error::custom(format!("Invalid UUID: {}", e)))?;

    collection.delete_one(doc! {"_id": uuid}).await?;

    Ok(Some(String::from("Product deleted successfully")))
}
