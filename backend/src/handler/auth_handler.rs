use axum::{Json, http::StatusCode};
use axum::response::IntoResponse;
use serde_json::json;
use crate::model::user_model::{RegisterRequest, LoginRequest, TokenResponse};
use crate::helper::jwt::generate_jwt;

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
    (StatusCode::CREATED, Json(json!({"message": "Registered"})))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login success", body = TokenResponse)
    )
)]
pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    let token = generate_jwt(&payload.username);
    tracing::info!("User logged in: {:?}", payload);
    tracing::info!("Generated token: {:?}", token);
    (StatusCode::OK, Json(TokenResponse { access_token: token }))
}
