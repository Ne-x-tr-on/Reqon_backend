use axum::{Router, routing::post};
use crate::handlers::auth_handler::{login_handler, register_handler};

pub fn auth_routes<StateType>() -> Router<StateType> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
}
