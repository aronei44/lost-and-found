
use axum::Error;
use axum::{
    Json, 
    http::StatusCode
};
use crate::helper::client_request::save_target_data;
use crate::model::photo_data_model::{CreatePhotoData, CollectedField, File_};
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
    let person_id_str = person_id.to_string();
    let files_len = files.len();
    tracing::info!("Received {} files for person_id: {}", files_len, person_id_str);
    let res = save_target_data(&person_id_str, files).await;
    if let Err(e) = res {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save target data: {}", e),
        );
    } else {
        let Ok(res) = res else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save target data".to_string(),
            );
        };
        tracing::info!("res from save_target_data: {:?}", res);
        // Build a JSON response with details about the upload
        let response = serde_json::json!({
            "bucket": res
                .get("bucket")
                .and_then(|v| v.as_str())
                .unwrap_or(""),
            "saved_files": res
                .get("saved_files")
                .cloned()
                .unwrap_or_else(|| serde_json::json!([]))
        });
        tracing::info!("Upload response: {:?}", response);
        let bucket = response["bucket"].as_str().unwrap_or("").to_string();
        let saved_files_array = response["saved_files"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let saved_files_len = &saved_files_array.len();
    
        for file_path_value in saved_files_array {
            if let Some(file_path) = file_path_value.as_str() {
                let photo = CreatePhoto {
                    bucket: bucket.clone(),
                    path: file_path.to_string(),
                };

                // Pastikan semua data sudah dimiliki, baru await
                let photo_result = create_photo(photo).await;
                if let Ok(p) = photo_result {
                    let _photo_data = CreatePhotoData {
                        photo_id: p.id,
                        lost_people_id: person_id,
                        place_id: None,
                    };
                    let _ = create_photo_data(_photo_data).await;
                }
            }
        }
    
        (
            StatusCode::OK,
            format!("Photo uploaded successfully: {} file(s)", saved_files_len)
        )

    }
}