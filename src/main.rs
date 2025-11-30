

use std::net::SocketAddr;

use axum::Router;
use dotenvy::dotenv;

use crate::routes::post::post_routes;

mod db;
mod handlers;
mod models;
mod routes;
mod auth;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = db::connect().await;

    let app = Router::new().merge(post_routes()).with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server berjalan di http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
