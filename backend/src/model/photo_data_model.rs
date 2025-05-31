
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use bytes::Bytes;

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


#[derive(Debug)]
pub struct CollectedField {
    pub name: Option<String>,
    pub file_name: Option<String>,
    pub content_type: Option<mime::Mime>,
    pub data: Bytes,
}

#[derive(ToSchema)]
pub struct File_ {
    #[schema(value_type = String, format = Binary)]
    pub files: Vec<u8>
}

#[derive(ToSchema)]
pub struct FileWithLongLat {
    pub latitude: String,
    pub longitude: String,
    #[schema(value_type = String, format = Binary)]
    pub files: Vec<u8>
}
