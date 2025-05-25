use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use chrono::NaiveDateTime;


#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub last_active: Option<NaiveDateTime>,
}


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

