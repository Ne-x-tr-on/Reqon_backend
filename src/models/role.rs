use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: i16,
    pub name: String,
}


#[derive(Serialize)]
pub struct RoleResponse {
    pub id: i16,
    pub name: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}