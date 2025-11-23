use crate::models::department::Department;
use axum::{extract::State, response::IntoResponse, Json, http::StatusCode};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn get_departments(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    let departments = sqlx::query_as!(Department, "SELECT id, name FROM core.departments")
        .fetch_all(&*pool) // deref Arc
        .await;

    match departments {
        Ok(d) => (StatusCode::OK, Json(d)),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<Department>::new()))
        }
    }
}

