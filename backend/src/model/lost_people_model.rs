use serde::{Serialize, Deserialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct LostPeople {
    pub id: i32,
    pub fullname: String,
    pub alias: Option<String>,
    pub gender: Option<String>,
    pub born_date: Option<chrono::NaiveDate>,
    pub last_condition: Option<String>,
    pub is_found: Option<bool>,
    pub lost_date: chrono::NaiveDate,
    pub found_date: Option<chrono::NaiveDate>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct MonitorPeople {
    pub user_username: String,
    pub lost_people_id: i32,
}


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CreateLostPeopleRequest {
    pub fullname: String,
    pub alias: Option<String>,
    pub gender: Option<String>,
    pub born_date: Option<chrono::NaiveDate>,
    pub last_condition: Option<String>,
    pub lost_date: chrono::NaiveDate,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UpdateLostPeopleRequest {
    pub id: i32,
    pub fullname: Option<String>,
    pub alias: Option<String>,
    pub gender: Option<String>,
    pub born_date: Option<chrono::NaiveDate>,
    pub last_condition: Option<String>,
    pub is_found: Option<bool>,
    pub lost_date: Option<chrono::NaiveDate>,
    pub found_date: Option<chrono::NaiveDate>,
}
