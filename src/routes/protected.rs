use axum::{Router, routing::get};
use mongodb::Database;

use crate::handlers::auth::me;

pub fn protected_routes() -> Router<Database> {
    Router::new().route("/", get(me))
}
