use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject, FromRow)]
pub struct Role {
    pub role_id: i16,
    pub role_name: String,
    pub parent: Option<i16>,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct RoleInput {
    pub role_name: String,
    pub parent: Option<i16>,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct RoleUpdateInput {
    pub role_id: i16,
    pub role_name: Option<String>,
    pub parent: Option<i16>,
}
