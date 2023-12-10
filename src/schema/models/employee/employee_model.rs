use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Clone, Enum, Copy, Serialize, Deserialize)]
pub enum Activity {
    ACTIVE,
    INACTIVE,
}

#[derive(Debug, Eq, PartialEq, Clone, Enum, Copy, Serialize, Deserialize)]
pub enum Gender {
    M,
    F,
}

#[derive(Deserialize, Debug, Clone, Serialize, SimpleObject, FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub photo: String,
    pub status: Activity,
    pub mobile: String,
    pub personal_email: String,
    pub gender: Gender,
    pub dob: Option<DateTime<Utc>>,
    pub employee_id: String,
    pub address: Value,
    pub qualification: Value,
    pub role: i16,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, Clone, Serialize, InputObject)]
pub struct EmployeeCreateInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub photo: String,
    pub mobile: String,
    pub personal_email: String,
    pub gender: Gender,
    pub dob: Option<DateTime<Utc>>,
    pub employee_id: String,
    pub address: Value,
    pub qualification: Value,
    pub role: i16,
}
