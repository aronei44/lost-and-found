use axum::{
    Router,
    response::IntoResponse
};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi; // Import the trait for openapi()

use crate::route::routes;
use crate::model::user_model::ApiDoc;

pub fn create_app() -> Router<()> {
    let swagger_ui = SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi());
    Router::new()
        .nest("/api", 
            Router::new().merge(routes())) // prefix all routes with /api
        .merge(swagger_ui) // Add Swagger UI
        .fallback(fallback_handler)
}

// Fallback handler for 404 errors
async fn fallback_handler() -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "404: Not Found")
}