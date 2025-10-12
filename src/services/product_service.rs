use actix_web::web;
use futures_util::stream::TryStreamExt;
use mongodb::Database;
use mongodb::bson::doc;
use uuid::Uuid;

use crate::models::product::{CreateProductDto, Product};

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
    new_product_data: web::Json<CreateProductDto>,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<Product>("products");

    let new_product = Product {
        _id: Uuid::new_v4(),
        name: new_product_data.name.to_string(),
        category: new_product_data.category,
        description: new_product_data.description.to_string(),
        discount: new_product_data.discount,
        images: new_product_data.images.clone(),
        price: new_product_data.price,
    };

    collection.insert_one(new_product).await?;

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
