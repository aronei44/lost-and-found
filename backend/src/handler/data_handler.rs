
use axum::{
    Json, 
    http::StatusCode
};
use crate::helper::client_request::{recognize_target, save_target_data};
use crate::model::photo_data_model::{CollectedField, CreatePhotoData, FileWithLongLat, File_};
use crate::model::photo_model::{CreatePhoto, Photo, PhotoWithLostPeople};
use crate::model::place_model::{self, CreatePlace, Place};
use crate::data::photos_data::{create_photo, create_photo_data, get_photos_by_person_id, get_places_by_person_id, get_photos_by_person_id_and_place_id, get_photos_with_lost_by_username, create_place};
use axum::extract::{
    Extension,
    Multipart,
    Path
};
use axum::response::IntoResponse;
use crate::model::user_model::User;
use mime;
use serde_json::Value;
use crate::helper::db::create_pool;


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
        if let Err(e) = save_data(&bucket, &saved_files_array, &vec![person_id], None).await {
            tracing::error!("Failed to save data: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to save data: {}", e),
            );
        }
    
        (
            StatusCode::OK,
            format!("Photo uploaded successfully: {} file(s)", saved_files_len)
        )

    }
}

async fn save_data(
    bucket: &str,
    saved_files: &Vec<Value>,
    ids: &Vec<i32>,
    place: Option<&CreatePlace>
) -> Result<(), String> {
    tracing::info!("Saving data with bucket: {}, files: {:?}, ids: {:?}", bucket, saved_files, ids);
    let mut place_id = None;
    
    let pool = create_pool().await.map_err(|e| e.to_string())?;
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    if let Some(p) = place {
        match create_place(&mut tx, p.clone()).await {
            Ok(created_place) => {
                place_id = Some(created_place.id);
            }
            Err(e) => {
                tracing::error!("Failed to create place: {}", e);
                return Err(e.to_string());
            }
        }
    }


    for person_id in ids {
        for file_path_val in saved_files {
            if let Some(file_path) = file_path_val.as_str() {
                let photo = CreatePhoto {
                    bucket: bucket.to_string(),
                    path: file_path.to_string(),
                };
                match create_photo(&mut tx, photo).await {
                    Ok(p) => {
                        let photo_data = CreatePhotoData {
                            photo_id: p.id,
                            lost_people_id: *person_id,
                            place_id: place_id,
                        };
                        if let Err(e) = create_photo_data(&mut tx, photo_data).await {
                            tracing::error!("Failed to create photo data: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to create photo: {}", e);
                    }
                }
            }
        }
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}


#[utoipa::path(
    post,
    path = "/api/data/recognize_target",
    request_body(
        content_type = "multipart/form-data",
        description = "Form field: file (binary)",
        content = FileWithLongLat
    ),
    responses(
        (status = 200, description = "Target recognized successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn recognize_target_handler(
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut longitude = String::from("0.0"); // Placeholder, replace with actual logic to get longitude
    let mut latitude = String::from("0.0"); // Placeholder, replace with actual logic to get latitude
    let mut file: CollectedField = CollectedField {
        name: None,
        file_name: None,
        content_type: None,
        data: bytes::Bytes::new(),
    };

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("files") {
            let file_name = field.file_name().map(|s| s.to_string());
            let content_type = field
                .content_type()
                .and_then(|ct| ct.parse::<mime::Mime>().ok());
            let data = match field.bytes().await {
                Ok(bytes) => bytes,
                Err(e) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        format!("Failed to read file: {}", e),
                    ));
                }
            };

            file = CollectedField {
                name: Some("file".to_string()),
                file_name,
                content_type,
                data,
            };
        } else if field.name() == Some("longitude") {
            if let Ok(value) = field.text().await {
                longitude = value.trim().to_string();
            }
        } else if field.name() == Some("latitude") {
            if let Ok(value) = field.text().await {
                latitude = value.trim().to_string();
            }
        }
    }


    // Call recognize_target with single file
    match recognize_target(file).await {
        Ok(recognized_data) => {
            tracing::info!("Recognized data: {:?}", recognized_data);
            // create json serde from {"bucket": String("lostfound"), "recognized": Array [String("1")], "saved_file": String("found_people/2025-05-31_08-48-24-589488.jpg")}
            let bucket = recognized_data
                .get("bucket")
                .and_then(|v| v.as_str())
                .unwrap_or("lostfound");
            // "recognized" is an array of strings, e.g., [String("1")]
            let recognized_ids = recognized_data
                .get("recognized")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            // Convert Vec<Value> of strings to Vec<Value> of numbers for save_data
            let recognized_ids: Vec<Value> = recognized_ids
                .into_iter()
                .filter_map(|v| {
                    if let Some(s) = v.as_str() {
                        s.parse::<i32>().ok().map(|n| Value::from(n))
                    } else {
                        None
                    }
                })
                .collect();
            let saved_files = recognized_data
                .get("saved_file")
                .map(|v| vec![v.clone()])
                .unwrap_or_default();
            let person_ids: Vec<i32> = recognized_ids
                .iter()
                .filter_map(|v| v.as_i64())
                .map(|v| v as i32)
                .collect();
            let place = CreatePlace {
                user_username: user.username.clone(),
                latitude: latitude.clone(),
                longitude: longitude.clone(),
            };
            if let Err(e) = save_data(bucket, &saved_files, &person_ids, Some(&place)).await {
                tracing::error!("Failed to save recognized data: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to save recognized data: {}", e),
                ));
            }
            return Ok((
                StatusCode::OK,
                format!("Recognized data: {:?}", recognized_data),
            ));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Recognition failed: {}", e),
            ));
        }
    }

}