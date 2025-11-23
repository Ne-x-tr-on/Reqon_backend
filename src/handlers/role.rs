use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::role::Role; // Assuming Role has fields: id, name
use crate::models::role::RoleResponse;
use crate::models::role::MessageResponse;


/// Get all roles
pub async fn get_roles_handler(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    let roles = sqlx::query_as!(Role, "SELECT id, name FROM core.roles")
        .fetch_all(&*pool)
        .await;

    match roles {
        Ok(roles) => {
            let resp: Vec<RoleResponse> = roles
                .into_iter()
                .map(|r| RoleResponse { id: r.id, name: r.name })
                .collect();
            (StatusCode::OK, Json(resp))
        }
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<RoleResponse>::new()))
        }
    }
}

/// Create a new role
#[derive(Deserialize)]
pub struct NewRoleRequest {
    pub name: String,
}

pub async fn create_role_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<NewRoleRequest>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO core.roles (name) VALUES ($1)",
        payload.name
    )
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(MessageResponse {
                message: "Role created successfully".into(),
            }),
        ),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MessageResponse {
                    message: e.to_string(),
                }),
            )
        }
    }
}
