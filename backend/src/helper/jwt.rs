use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_jwt(user_id: &str) -> String {
    let expiration = chrono::Utc::now().timestamp() as usize + 3600;
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default".to_string());
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}