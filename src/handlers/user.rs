use crate::models::user::User;
use std::sync::Arc;

use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};

use sqlx::PgPool;
use crate::services::user_service::get_all_users;
use crate::services::user_service::create_new_user;
use crate::models::user::NewUser;

pub async fn get_users_handler(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match get_all_users(&pool).await {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<User>::new()))
        }
    }
}

pub async fn create_user_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<NewUser>
) -> impl IntoResponse {
    match create_new_user(&pool, payload).await {
        Ok(_) => (StatusCode::CREATED, Json("User created successfully".to_string())),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())),
    }
}


