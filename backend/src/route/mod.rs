use axum::{Router, routing::post, middleware};
use crate::handler::auth_handler::{register, login, refresh_token};

pub fn routes() -> Router {
    Router::new()
        .nest("/auth", Router::new()
            .route("/register", post(register))
            .route("/login", post(login))
            .route(
                "/refresh",
                post(refresh_token)
                    .layer(middleware::from_fn(crate::helper::auth::authorization_middleware))
            )
        )
}