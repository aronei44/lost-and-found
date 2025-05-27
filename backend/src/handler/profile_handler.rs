use axum::{
    Json, 
    http::{
        StatusCode,
        header::HeaderMap
    }
};
use crate::{helper::jwt::get_user_from_header, model::profile_model::{ProfileResponse, UpdateProfileRequest}};
use crate::data::profile_data::{create_profile, get_profile_by_username, update_profile};

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
pub async fn update_profile_handler( header: HeaderMap, Json(payload): Json<UpdateProfileRequest>) -> Result<Json<ProfileResponse>, (StatusCode, Json<serde_json::Value>)> {
    let user = get_user_from_header(header)
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))))?;

    match get_profile_by_username(&user.claims.username).await {
        Ok(_) => {
            let updated_profile = update_profile(&user.claims.username, &payload).await.map_err(|e| {
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
            let new_profile = create_profile(&user.claims.username, &payload).await.map_err(|e| {
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
