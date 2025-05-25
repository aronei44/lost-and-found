use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::{data::user_data::get_user_by_username, helper::jwt::decode_jwt};

pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message
        }));
        (self.status_code, body).into_response()
    }
}

pub async fn authorization_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let auth_header = req
        .headers_mut()
        .get(axum::http::header::AUTHORIZATION)
        .ok_or(AuthError {
            message: "Please add the JWT token to the header".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?
        .to_str()
        .map_err(|_| AuthError {
            message: "Invalid header format".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?;

    let mut parts = auth_header.split_whitespace();
    let (bearer, token) = (parts.next(), parts.next());

    if bearer != Some("Bearer") || token.is_none() {
        return Err(AuthError {
            message: "Invalid authorization header".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let token = token.unwrap();
    let token_data = decode_jwt(token).map_err(|_| AuthError {
        message: "Unable to decode token".to_string(),
        status_code: StatusCode::UNAUTHORIZED,
    })?;

    let current_user = get_user_by_username(&token_data.claims.sub)
        .await
        .map_err(|_| AuthError {
            message: "You are not an authorized user".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
