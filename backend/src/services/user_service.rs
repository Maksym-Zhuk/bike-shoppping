use actix_web::web;
use bson::{Uuid, doc, to_document};
use mongodb::Database;

use crate::{
    dto::{auth::UserInfo, user::UpdateUserDto},
    errors::AppErrors,
    models::user::User,
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
