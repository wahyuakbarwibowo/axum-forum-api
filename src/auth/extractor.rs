use std::env;

use axum::{extract::FromRequestParts, http::StatusCode};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::auth::claims::Claims;

pub struct AuthClaims(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync, // state not needed here
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
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

        let secret = env::var("JWT_SECRET")
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "JWT_SECRET not set"))?;
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
