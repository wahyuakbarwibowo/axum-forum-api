use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{EncodingKey, Header, encode};
use mongodb::{
    Database,
    bson::{DateTime, oid::ObjectId},
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        claims::{self, Claims},
        extractor::AuthClaims,
    },
    models::user::User,
};

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: usize,
}

#[derive(Serialize)]
pub struct MeResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub async fn register(
    State(db): State<Database>,
    Json(payload): Json<RegisterPayload>,
) -> Result<(StatusCode, Json<RegisterResponse>), (StatusCode, String)> {
    let col = db.collection::<User>("users");

    // check if email exists
    if let Ok(Some(_)) = col.find_one(doc! { "email": &payload.email }).await {
        return Err((StatusCode::CONFLICT, "Email already registered".to_string()));
    }

    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_user = User {
        id: Some(ObjectId::new()),
        username: payload.username.clone(),
        email: payload.email.clone(),
        password_hash,
        created_at: Some(DateTime::now()),
    };

    col.insert_one(&new_user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            id: new_user.id.unwrap().to_hex(),
            username: new_user.username,
            email: new_user.email,
        }),
    ))
}

pub async fn login(
    State(db): State<Database>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let col = db.collection::<User>("users");

    let user = col
        .find_one(doc! { "email": &payload.email })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    // verify password
    let valid = verify(&payload.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // create token
    let secret = env::var("JWT_SECRET").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "JWT_SECRET not set".to_string(),
        )
    })?;
    let expiry_seconds: usize = env::var("JWT_EXPIRY_SECONDS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3600);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let claims = Claims {
        sub: user.id.unwrap().to_hex(),
        iat: now,
        exp: now + expiry_seconds,
    };

    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer",
        expires_in: expiry_seconds,
    }))
}

pub async fn me(
    AuthClaims(claims): AuthClaims,
    State(db): State<Database>,
) -> Result<Json<MeResponse>, (StatusCode, String)> {
    let col = db.collection::<User>("users");

    let oid = ObjectId::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user id".to_string()))?;

    let user 
}
