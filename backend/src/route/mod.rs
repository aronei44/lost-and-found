use axum::{Router, routing::post};
use crate::handler::auth_handler::{register, login};

pub fn routes() -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}