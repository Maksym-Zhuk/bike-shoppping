use anyhow::{Result, anyhow};
use bson::{Uuid, doc};
use mongodb::Database;

use crate::{dto::auth::UserInfo, models::user::User};

pub async fn me(db: &Database, user_id: String) -> Result<UserInfo> {
    let collection = db.collection::<User>("users");

    let uuid = Uuid::parse_str(user_id)
        .map_err(|e| mongodb::error::Error::custom(format!("Invalid UUID: {}", e)))?;

    let user = collection.find_one(doc! {"_id": uuid}).await?;

    match user {
        Some(user) => Ok(UserInfo {
            id: user._id.to_string(),
            email: user.email,
            name: user.name,
        }),
        None => Err(anyhow!("User not found")),
    }
}
