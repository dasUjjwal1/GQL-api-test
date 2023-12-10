use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, Enum, Copy, PartialEq, Eq)]
pub enum UserStatus {
    ACTIVE,
    INACTIVE,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<i8>,
    pub area: Value,
    pub photo: String,
    pub status: UserStatus,
    pub user_name: String,
    pub parent_user_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<i8>,
    pub area: Value,
    pub user_name: String,
    pub parent_user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct LoginInput {
    pub user_name: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject, FromRow)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: Option<i8>,
    pub area: Value,
    pub photo: String,
    pub status: UserStatus,
    pub user_name: String,
    pub parent_user_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
