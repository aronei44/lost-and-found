use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct Photo {
    pub id: i32,
    pub bucket: String,
    pub path: String,
}


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CreatePhoto {
    pub bucket: String,
    pub path: String,
}
