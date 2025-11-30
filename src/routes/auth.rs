use axum::{Router, routing::Route};
use mongodb::Database;

use crate::handlers::{
    auth::{login, register},
    post,
};

pub fn auth_routes() -> Router<Database> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
