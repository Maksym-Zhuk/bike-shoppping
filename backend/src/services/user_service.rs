use bson::{Uuid, doc};
use mongodb::Database;

use crate::{dto::auth::UserInfo, errors::AppErrors, models::user::User};

pub async fn me(db: &Database, user_id: String) -> Result<UserInfo, AppErrors> {
    let collection = db.collection::<User>("users");

    let uuid = Uuid::parse_str(user_id).map_err(|_| AppErrors::InvalidUUID)?;

    let user = collection.find_one(doc! {"_id": uuid}).await?;

    match user {
        Some(user) => Ok(UserInfo {
            id: user._id.to_string(),
            email: user.email,
            name: user.name,
        }),
        None => Err(AppErrors::NotFound("User".to_string())),
    }
}
