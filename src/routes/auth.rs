use axum::{Router, routing::post};
use crate::handlers::auth::{login_handler, register_handler};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
}
