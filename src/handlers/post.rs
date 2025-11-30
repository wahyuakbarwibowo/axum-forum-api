use crate::models::post::Post;
use axum::{Json, extract::State};
use mongodb::{
    Database,
    bson::{DateTime, doc, oid::ObjectId},
};
use futures_util::TryStreamExt;

pub async fn create_post(State(db): State<Database>, Json(mut new_post): Json<Post>) -> Json<Post> {
    let col = db.collection::<Post>("posts");

    new_post.id = Some(ObjectId::new());
    new_post.created_at = Some(DateTime::now());

    col.insert_one(&new_post).await.unwrap();

    Json(new_post)
}

pub async fn get_posts(State(db): State<Database>) -> Json<Vec<Post>> {
    let col = db.collection::<Post>("posts");
    let mut cursor = col.find(doc! {}).await.unwrap();

    let mut posts = vec![];
    while let Some(p) = cursor.try_next().await.unwrap() {
        posts.push(p);
    }
    Json(posts)
}
