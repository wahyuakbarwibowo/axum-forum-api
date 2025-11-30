use std::env;

use mongodb::{Client, Database};

pub async fn connect() -> Database {
    let uri = env::var("MONGO_URI").expect("MONGO_URI not found");
    let client = Client::with_uri_str(uri)
        .await
        .expect("Fail to connect MongoDB");
    client.database("forum_db")
}
