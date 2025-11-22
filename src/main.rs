// src/main.rs
use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
};
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
    // Load environment variables from .env
    dotenv().ok();

    // Initialize the Postgres pool
    let pool = db::pool::init_pool().await?;

    // Build our application with routes
    let app = Router::new()
        // Health check
        .route("/health", get(|| async { "Reqon backend is running!" }))
        // Nest department/user/auth routes
        .nest("/auth", routes::auth::auth_routes())
        .nest("/users", routes::users::user_routes())
        .nest("/dashboard", routes::dashboard::dashboard_routes())
        .nest("/tasks", routes::tasks::task_routes())
        // Add middleware, logging, etc.
        .layer(TraceLayer::new_for_http())
        // Attach the database pool as state
        .with_state(pool);

    // Read port from env or default to 8080
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Reqon backend running on http://{}", addr);

    // Run the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
