use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: Option<String>,
    pub employee_number: String,
    pub dob: chrono::NaiveDate,
    pub gender: Option<String>,
    pub department_id: i16,
    pub role_id: i16,
    pub created_at: chrono::NaiveDateTime,
}
use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::services::user_service::get_all_users;
use crate::services::user_service::create_new_user;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub full_name: String,
    pub email: String,
}

pub async fn get_users_handler(State(pool): State<PgPool>) -> impl IntoResponse {
    match get_all_users(&pool).await {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

#[derive(Deserialize)]
pub struct NewUserRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub department_id: i16,
    pub role_id: i16,
}

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<NewUserRequest>
) -> impl IntoResponse {
    match create_new_user(&pool, payload).await {
        Ok(_) => (StatusCode::CREATED, Json("User created successfully")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())),
    }
}
