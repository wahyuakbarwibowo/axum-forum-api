use axum::Router;
use mongodb::Database;

use crate::routes::{auth::auth_routes, post::post_routes, protected::protected_routes};

pub mod auth;
pub mod post;
pub mod protected;

pub fn api_routes() -> Router<Database> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/me", protected_routes())
        .nest("/posts", post_routes())
}
