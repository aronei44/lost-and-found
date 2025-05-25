use axum::{Router, routing::{post, put}, middleware};
use crate::handler::{
    auth_handler::{register, login, refresh_token},
    profile_handler::update_profile_handler
};

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
        .route("/profile", put(update_profile_handler)
            .layer(middleware::from_fn(crate::helper::auth::authorization_middleware))
    )
}