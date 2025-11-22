use crate::models::{User, NewUser, LoginUser, Role, Department};

use axum::{extract::Extension, response::IntoResponse, Json, http::StatusCode};
use sqlx::PgPool;
use crate::models::Department;

/// Get all departments
pub async fn get_departments(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let departments = sqlx::query_as!(Department, "SELECT id, name FROM core.departments")
        .fetch_all(&pool)
        .await;

    match departments {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch departments").into_response(),
    }
}
