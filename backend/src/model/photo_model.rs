use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct Photo {
    pub id: i32,
    pub bucket: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct PhotoWithLostPeople {
    pub id: i32,
    pub bucket: String,
    pub path: String,
    pub person_id: i32,
    pub fullname: String,
    pub alias: Option<String>,
    pub gender: Option<String>,
    pub born_date: Option<chrono::NaiveDate>,
    pub last_condition: Option<String>,
    pub is_found: Option<bool>,
    pub lost_date: chrono::NaiveDate,
    pub found_date: Option<chrono::NaiveDate>,
}


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CreatePhoto {
    pub bucket: String,
    pub path: String,
}
