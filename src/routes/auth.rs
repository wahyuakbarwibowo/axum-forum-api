use axum::{Router, routing::post};
use mongodb::Database;

use crate::handlers::auth::{login, register};

pub fn auth_routes() -> Router<Database> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
