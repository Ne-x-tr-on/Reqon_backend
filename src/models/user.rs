use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: Option<String>,
    pub employee_number: String,
    pub dob: NaiveDate,
    pub gender: Option<String>,
    pub department_id: Option<i16>, // <-- check DB column, maybe Option<i16>
    pub role_id: Option<i16>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub employee_number: String,
    pub dob: String, // parsed into NaiveDate
    pub gender: Option<String>,
    pub department_id: Option<i16>, // <-- check DB column, maybe Option<i16>
    pub role_id: Option<i16>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub full_name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NewUserRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub department_id: i16,
    pub role_id: i16,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub dob: String,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub department_id: i16,
    pub role_id: i16,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
