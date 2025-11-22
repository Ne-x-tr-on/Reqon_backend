use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: Option<String>,
    pub employee_number: String,
    pub dob: NaiveDate,
    pub gender: Option<String>,
    pub department_id: i16,
    pub role_id: i16,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub employee_number: String,
    pub dob: String,
    pub gender: Option<String>,
    pub department_id: i16,
    pub role_id: i16,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
