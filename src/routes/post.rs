use axum::{Router, routing::post};
use mongodb::Database;

use crate::handlers::post::{create_post, get_posts};

pub fn post_routes() -> Router<Database> {
    Router::new().route("/", post(create_post).get(get_posts))
}
