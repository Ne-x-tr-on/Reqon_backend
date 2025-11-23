// use crate::models::{User, Department, Task, Role, NewUser, LoginUser};

// src/main.rs
use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router, routing::get};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;

mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Wrap pool in Arc
    let pool = Arc::new(db::pool::init_pool().await?);

    let app = Router::new()
        .route("/health", get(|| async { "Reqon backend is running!" }))
        .nest("/auth", routes::auth::auth_routes())   // nested routers keep unit state
        .nest("/users", routes::users::auth_routes()) // fixed typo
        .layer(TraceLayer::new_for_http())
        .with_state(pool.clone()); // attach Arc<PgPool> at top-level

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Reqon backend running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

