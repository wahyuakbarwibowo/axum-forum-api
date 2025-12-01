use std::env;

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::auth::claims::Claims;

pub struct AuthClaims(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync, // state not needed here
    Self: Sized + 'static,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // ambil header Authorization
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "Invalid Authorization header".to_string(),
                )
            })?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid Authorization scheme".to_string(),
            ));
        }

        let token = auth_header.trim_start_matches("Bearer ").trim();

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        let token_data = decode::<Claims>(
            token,
            &decoding_key,
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?;

        Ok(AuthClaims(token_data.claims))
    }
}
