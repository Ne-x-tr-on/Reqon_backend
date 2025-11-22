use axum::{Router, routing::{get, post}};
use crate::handlers::tasks::{get_tasks_handler, create_task_handler};

pub fn task_routes() -> Router {
    Router::new()
        .route("/", get(get_tasks_handler))
        .route("/create", post(create_task_handler))
}
