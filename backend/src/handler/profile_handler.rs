use axum::{
    Json, 
    http::StatusCode
};
use crate::model::profile_model::{ProfileResponse, UpdateProfileRequest};
use crate::data::profile_data::{create_profile, get_profile_by_username, update_profile};
use axum::extract::Extension;
use crate::model::user_model::User;


#[utoipa::path(
    put,
    path = "/api/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile Updated", body = ProfileResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_profile_handler( Extension(current_user): Extension<User>, Json(payload): Json<UpdateProfileRequest>) -> Result<Json<ProfileResponse>, (StatusCode, Json<serde_json::Value>)> {

    match get_profile_by_username(&current_user.username).await {
        Ok(_) => {
            let updated_profile = update_profile(&current_user.username, &payload).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
            })?;
            Ok(Json(ProfileResponse {
                username: updated_profile.username,
                full_name: updated_profile.full_name,
                email: updated_profile.email,
                phone: updated_profile.phone,
                address: updated_profile.address,
            }))
        },
        Err(_) => {
            let new_profile = create_profile(&current_user.username, &payload).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
            })?;
            Ok(Json(ProfileResponse {
                username: new_profile.username,
                full_name: new_profile.full_name,
                email: new_profile.email,
                phone: new_profile.phone,
                address: new_profile.address,
            }))
        }
    }
    
}


#[utoipa::path(
    get,
    path = "/api/profile",
    responses(
        (status = 200, description = "Profile Retrieved", body = ProfileResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_profile_handler(Extension(current_user): Extension<User>) -> Result<Json<ProfileResponse>, (StatusCode, Json<serde_json::Value>)> {
    match get_profile_by_username(&current_user.username).await {
        Ok(profile) => Ok(Json(ProfileResponse {
            username: profile.username,
            full_name: profile.full_name,
            email: profile.email,
            phone: profile.phone,
            address: profile.address,
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )),
    }
}