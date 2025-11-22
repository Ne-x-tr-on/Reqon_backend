use axum::{extract::{State, Json}, response::IntoResponse, http::StatusCode};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use crate::services::task_service;

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct NewTaskRequest {
    pub title: String,
    pub description: String,
}

pub async fn get_tasks_handler(State(pool): State<PgPool>) -> impl IntoResponse {
    match task_service::get_all_tasks(&pool).await {
        Ok(tasks) => (StatusCode::OK, Json(tasks)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

pub async fn create_task_handler(State(pool): State<PgPool>, Json(payload): Json<NewTaskRequest>) -> impl IntoResponse {
    match task_service::create_task(&pool, payload).await {
        Ok(_) => (StatusCode::CREATED, Json("Task created")),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())),
    }
}
