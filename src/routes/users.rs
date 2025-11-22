use axum::{Router, routing::{get, post}};
use crate::handlers::users::{get_users_handler, create_user_handler};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(get_users_handler))
        .route("/create", post(create_user_handler))
}
