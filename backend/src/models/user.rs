use serde::{Deserialize, Serialize};
// use ts_rs::TS;
// use utoipa::ToSchema;
use uuid::Uuid;

use crate::{dto::auth::UserInfo, models::role::Role};

#[derive(/*TS,*/ Serialize, Deserialize, Clone /*ToSchema*/)]
// #[ts(export, export_to = "../../db_types/User.d.ts")]
pub struct User {
    // #[ts(type = "string")]
    // #[schema(value_type = String)]
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub _id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            id: user._id.to_string(),
            name: user.name,
            email: user.email,
            role: user.role,
        }
    }
}
