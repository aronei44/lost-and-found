use axum::http::HeaderMap;
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, TokenData, errors::Error};
use serde::{Serialize, Deserialize};
use std::env;
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub token_type: String,
    pub username: String
}

pub fn generate_access_token(user_id: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        token_type: "access".into(),
        username: user_id.to_string(),
    };

    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default".into());
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn generate_refresh_token(user_id: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        token_type: "refresh".into(),
        username: user_id.to_string(),
    };

    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default".into());
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default".into());
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

pub fn get_user_from_header(header: HeaderMap) -> Result<TokenData<Claims>, Error> {
    let token = header.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))?;

    decode_jwt(token)
}