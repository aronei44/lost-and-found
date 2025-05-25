use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use chrono::NaiveDateTime;


#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct Profile {
    pub username: String,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UpdateProfileRequest {
    pub username: String,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ProfileResponse {
    pub username: String,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>
}
