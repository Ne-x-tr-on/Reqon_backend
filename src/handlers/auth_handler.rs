use std::sync::Arc;
use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
// use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::services::auth_service;
use crate::models::user::RegisterRequest;
use crate::models::user::RegisterResponse;
use crate::models::user::LoginResponse;
use crate::models::user::LoginRequest;


pub async fn register_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<RegisterRequest>
) -> impl IntoResponse {
    match auth_service::register_user(&pool, payload).await {
        Ok(_) => (StatusCode::CREATED, Json(RegisterResponse { message: "User registered".into() })),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(RegisterResponse { message: e.to_string() })),
    }
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
