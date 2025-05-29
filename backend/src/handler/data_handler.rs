
use axum::{
    Json, 
    http::StatusCode
};
use crate::model::photo_data_model::CreatePhotoData;
use crate::model::photo_model::{CreatePhoto, Photo, PhotoWithLostPeople};
use crate::model::place_model::Place;
use crate::data::photos_data::{create_photo, create_photo_data, get_photos_by_person_id, get_places_by_person_id, get_photos_by_person_id_and_place_id, get_photos_with_lost_by_username};
use axum::extract::{
    Extension,
    Multipart,
    Path
};
use axum::response::IntoResponse;
use crate::model::user_model::User;
use mime;
use bytes::Bytes;
use utoipa::ToSchema;



#[utoipa::path(
    get,
    path = "/api/data/photos/{person_id}",
    params(
        ("person_id" = i32, Path, description = "Person ID")
    ),
    responses(
        (status = 200, description = "List of Photo", body = Vec<Photo>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_photos_by_person_id_handler(
    Path(person_id): Path<i32>,
) -> Result<Json<Vec<Photo>>, (StatusCode, String)> {
    match get_photos_by_person_id(person_id).await {
        Ok(photos) => Ok(Json(photos)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/data/places/{person_id}",
    params(
        ("person_id" = i32, Path, description = "Person ID")
    ),
    responses(
        (status = 200, description = "List of Place", body = Vec<Place>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_places_by_person_id_handler(
    Path(person_id): Path<i32>,
) -> Result<Json<Vec<Place>>, (StatusCode, String)> {
    match get_places_by_person_id(person_id).await {
        Ok(places) => Ok(Json(places)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/data/photos/{person_id}/{place_id}",
    params(
        ("person_id" = i32, Path, description = "Person ID"),
        ("place_id" = i32, Path, description = "Place ID")
    ),
    responses(
        (status = 200, description = "List of Photo", body = Vec<Photo>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_photos_by_person_id_and_place_id_handler(
    Path((person_id, place_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<Photo>>, (StatusCode, String)> {
    match get_photos_by_person_id_and_place_id(person_id, place_id).await {
        Ok(photos) => Ok(Json(photos)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/data/monitoring",
    responses(
        (status = 200, description = "List of Photo with Lost People", body = Vec<PhotoWithLostPeople>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_photos_with_lost_by_username_handler(
    Extension(user): Extension<User>,
) -> Result<Json<Vec<PhotoWithLostPeople>>, (StatusCode, String)> {
    match get_photos_with_lost_by_username(&user.username).await {
        Ok(photos) => Ok(Json(photos)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}


#[derive(Debug)]
struct CollectedField {
    name: Option<String>,
    file_name: Option<String>,
    content_type: Option<mime::Mime>,
    data: Bytes,
}

#[derive(ToSchema)]
struct File_ {
    #[schema(value_type = String, format = Binary)]
    files: Vec<u8>
}

#[utoipa::path(
    post,
    path = "/api/data/upload_photo/{person_id}",
    params(
        ("person_id" = i32, Path, description = "Person ID")
    ),
    request_body(
        content_type = "multipart/form-data",
        description = "Form fields: name (string), files (binary array)",
        content= File_
    ),
    responses(
        (status = 200, description = "Photo uploaded successfully"),
        (status = 400, description = "Invalid input")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn upload_photo_handler(
    Path(person_id): Path<i32>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut files: Vec<CollectedField> = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("files") => {
                let file_name = field.file_name().map(|s| s.to_string());
                let content_type = field.content_type().and_then(|ct| ct.parse::<mime::Mime>().ok());
                let data = match field.bytes().await {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        return (
                            StatusCode::BAD_REQUEST,
                            format!("Failed to read file: {}", e),
                        );
                    }
                };

                files.push(CollectedField {
                    name: Some("files".to_string()),
                    file_name,
                    content_type,
                    data,
                });
            },
            _ => {}
        }
    }

    if files.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            format!("files are required"),
        );
    }

    // Example response data, can be sent as JSON if needed
    let response = serde_json::json!({
        "bucket": "lostfound",
        "saved_files": ["/apagitu.jpg", "/apagitup.jpg"]
    });

    // Simulate saving the photo and photo data
    for res in response["saved_files"].as_array().unwrap_or(&vec![]) {
        let file_path = res.as_str().unwrap_or("");
        let photo = CreatePhoto {
            bucket: response["bucket"].as_str().unwrap_or("").to_string(),
            path: file_path.to_string(),
        };
        if let Ok(p) = create_photo(photo).await {
            let _photo_data = CreatePhotoData {
                photo_id: p.id,
                lost_people_id: person_id,
                place_id: None, // Assuming no place ID for this example
            };
            let _ = create_photo_data(_photo_data).await;
        }
    }

    // Bisa diteruskan ke service lain nanti
    (
        StatusCode::OK,
        format!("Photo uploaded successfully: {} file(s)", files.len()),
    )
}