use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(TS, Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema, Copy)]
#[ts(export, export_to = "../../db_types/Role.d.ts")]
pub enum Role {
    Admin,
    User,
}
