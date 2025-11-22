use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
}
