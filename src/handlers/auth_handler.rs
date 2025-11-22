use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::services::auth_service;

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

pub async fn register_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>
) -> impl IntoResponse {
    match auth_service::register_user(&pool, payload).await {
        Ok(_) => (StatusCode::CREATED, Json(RegisterResponse { message: "User registered".into() })),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(RegisterResponse { message: e.to_string() })),
    }
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

pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>
) -> impl IntoResponse {
    match auth_service::login_user(&pool, payload).await {
        Ok(token) => (StatusCode::OK, Json(LoginResponse { token })),
        Err(e) => (StatusCode::UNAUTHORIZED, Json(LoginResponse { token: e.to_string() })),
    }
}
