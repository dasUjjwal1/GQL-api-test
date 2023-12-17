use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Clone, Enum, Copy, Serialize, Deserialize)]
pub enum Gender {
    M,
    F,
    O,
}

#[derive(Debug, Clone, Serialize, SimpleObject, FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub photo: String,
    pub status: bool,
    pub mobile: String,
    pub personal_email: String,
    pub gender: Option<i16>,
    pub dob: Option<NaiveDate>,
    pub employee_id: String,
    pub address: Value,
    pub qualification: Value,
    pub role: Option<Vec<i16>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, Clone, InputObject)]
pub struct EmployeeCreateInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub photo: Option<String>,
    pub mobile: String,
    pub personal_email: Option<String>,
    pub gender: Option<String>,
    pub dob: Option<NaiveDate>,
    pub employee_id: String,
    pub address: Option<Value>,
    pub qualification: Option<Value>,
    pub role: Option<Vec<i16>>,
}

#[derive(Debug, Clone, Serialize, SimpleObject, FromRow)]
pub struct EmployeeResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub photo: String,
    pub status: bool,
    pub mobile: String,
    pub personal_email: String,
    pub gender: Option<Gender>,
    pub dob: Option<NaiveDate>,
    pub employee_id: String,
    pub address: Value,
    pub qualification: Value,
    pub role: Option<Vec<i16>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
