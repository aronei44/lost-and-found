use axum::{Json, http::StatusCode};
use axum::response::IntoResponse;
use crate::model::user_model::{RegisterRequest, LoginRequest, TokenResponse, RefreshRequest};
use crate::helper::jwt::{decode_jwt, generate_access_token, generate_refresh_token};
use crate::data::user_data::{create_user, get_user_by_username, update_user_last_active, verify_password};
use serde_json::json;

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered")
    )
)]
pub async fn register(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    tracing::info!("Registering user: {:?}", payload);
    // Check if the user already exists
    match get_user_by_username(&payload.username).await {
        Ok(_) => {
            tracing::warn!("User already exists: {:?}", payload.username);
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "User already exists"})));
        }
        Err(_) => {
            tracing::info!("User does not exist, proceeding with registration");
        }
    }

    // Create the user
    let user = create_user(&payload.username, &payload.password).await;
    match user {
        Ok(user) => {
            tracing::info!("User created: {:?}", user);
            (StatusCode::CREATED, Json(serde_json::json!({"message": "Registered"})))
        }
        Err(e) => {
            tracing::error!("Error creating user: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Internal server error"})));
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login success", body = TokenResponse)
    )
)]
pub async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<TokenResponse>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("Logging in user: {:?}", payload);

    let user = match get_user_by_username(&payload.username).await {
        Ok(user) => user,
        Err(_) => {
            tracing::warn!("User not found: {:?}", payload.username);
            return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))));
        }
    };

    match verify_password(&payload.password, &user.password).await {
        Ok(valid) if valid => (),
        Ok(_) => {
            tracing::warn!("Invalid password for user: {:?}", payload.username);
            return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))));
        }
        Err(e) => {
            tracing::error!("Error verifying password: {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Internal server error"}))));
        }
    }

    if let Err(e) = update_user_last_active(user.id).await {
        tracing::error!("Error updating last active time: {:?}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Internal server error"}))));
    }

    let access_token = generate_access_token(&payload.username);
    let refresh_token = generate_refresh_token(&payload.username);

    Ok(Json(TokenResponse { access_token, refresh_token }))
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = TokenResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn refresh_token(
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, (StatusCode, String)> {

    let token_data = decode_jwt(&payload.refresh_token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid refresh token".into()))?;

    if token_data.claims.token_type != "refresh" {
        return Err((StatusCode::UNAUTHORIZED, "Not a refresh token".into()));
    }

    let new_access = generate_access_token(&token_data.claims.sub);

    // Kembaliannya juga TokenResponse langsung
    Ok(Json(TokenResponse {
        access_token: new_access,
        refresh_token: payload.refresh_token.clone(), // bisa juga generate baru
    }))
}
