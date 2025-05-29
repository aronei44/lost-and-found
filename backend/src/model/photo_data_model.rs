
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct PhotoData {
    pub id: i32,
    pub photo_id: i32,
    pub lost_people_id: i32,
    pub place_id: Option<i32>
}


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CreatePhotoData {
    pub photo_id: i32,
    pub lost_people_id: i32,
    pub place_id: Option<i32>,
}
