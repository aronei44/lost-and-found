use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct Place {
    pub id: i32,
    pub user_username: String,
    pub latitude: String,
    pub longitude: String,
    pub created_at: Option<chrono::NaiveDateTime>
}


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CreatePlace {
    pub user_username: String,
    pub latitude: String,
    pub longitude: String,
}
