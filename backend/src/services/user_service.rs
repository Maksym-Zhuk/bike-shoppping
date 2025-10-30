use actix_web::web;
use bson::{doc, to_document, Uuid};
use futures_util::TryStreamExt;
use mongodb::Database;

use crate::{
    dto::{auth::UserInfo, user::UpdateUserDto},
    errors::AppErrors,
    models::{order::Order, user::User},
};

pub async fn me(db: &Database, user_id: String) -> Result<UserInfo, AppErrors> {
    let collection = db.collection::<User>("users");

    let uuid = Uuid::parse_str(user_id).map_err(|_| AppErrors::InvalidUUID)?;

    let user = collection.find_one(doc! {"_id": uuid}).await?;

    match user {
        Some(user) => Ok(UserInfo {
            id: user._id.to_string(),
            email: user.email,
            name: user.name,
            role: user.role,
        }),
        None => Err(AppErrors::NotFound("User".to_string())),
    }
}

pub async fn update_user(
    db: &Database,
    user_id: String,
    new_data: web::Json<UpdateUserDto>,
) -> Result<String, AppErrors> {
    let collection = db.collection::<User>("users");

    let uuid = Uuid::parse_str(user_id).map_err(|_| AppErrors::InvalidUUID)?;

    let update_doc = to_document(&new_data)?;

    let result = collection
        .update_one(doc! {"_id": uuid}, doc! {"$set": update_doc})
        .await?;

    if result.matched_count == 0 {
        return Err(AppErrors::NotFound("User".to_string()));
    };

    Ok(String::from("User updated successfully"))
}

pub async fn delete_user(db: &Database, user_id: String) -> Result<String, AppErrors> {
    let collection = db.collection::<User>("users");

    let uuid = Uuid::parse_str(user_id).map_err(|_| AppErrors::InvalidUUID)?;

    let result = collection.delete_one(doc! {"_id": uuid}).await?;

    if result.deleted_count == 0 {
        return Err(AppErrors::NotFound("User".to_string()));
    }

    Ok(String::from("User deleted successfully"))
}

pub async fn get_all_users(db: &Database) -> Result<Vec<UserInfo>, AppErrors> {
    let collection = db.collection::<User>("users");

    let mut cursor = collection.find(doc! {}).await?;
    let mut users = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        users.push(result);
    }

    let response: Vec<UserInfo> = users.into_iter().map(Into::into).collect();

    Ok(response)
}

pub async fn get_my_orders(db: &Database, user_id: String) -> Result<Vec<Order>, AppErrors> {
    let collection = db.collection::<Order>("orders");

    let cursor = collection.find(doc! { "customer_id": &user_id }).await?;

    let orders: Vec<Order> = cursor.try_collect().await?;

    Ok(orders)
}
