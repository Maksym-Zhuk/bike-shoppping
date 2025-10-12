use actix_web::web;
use anyhow::{Result, anyhow};
use bson::{Binary, Bson};
use futures_util::stream::TryStreamExt;
use mongodb::Database;
use mongodb::bson::doc;
use uuid::Uuid;

use crate::models::{
    order::{CreateOrderDto, Order},
    product::Product,
};

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
