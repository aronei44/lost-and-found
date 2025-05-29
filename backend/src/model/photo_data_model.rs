use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::model::{
    lost_people_model::LostPeople,
    photo_model::Photo,
    place_model::Place,
};
#[derive(Serialize, Deserialize, ToSchema, Debug, sqlx::FromRow, Clone)]
pub struct PhotoData {
    pub id: i32,
    pub photo_id: i32,
    pub lost_people_id: i32,
    pub place_id: Option<i32>,
    pub photo: Option<Photo>,
    pub lost_people: Option<LostPeople>,
    pub place: Option<Place>
}


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CreatePhotoData {
    pub photo_id: i32,
    pub lost_people_id: i32,
    pub place_id: Option<i32>,
}
