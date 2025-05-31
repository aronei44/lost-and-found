use axum::{
    Router,
    response::IntoResponse
};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi; // Import the trait for openapi()

use crate::route::routes;
use crate::api_doc::ApiDoc;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};
pub fn create_app() -> Router<()> {
    let swagger_ui = SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi());
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api", 
            Router::new().merge(routes())) // prefix all routes with /api
        .merge(swagger_ui) // Add Swagger UI
        .layer(cors) // Add CORS layer
        .layer(TraceLayer::new_for_http()) // Add tracing layer
        .fallback(fallback_handler)
}

// Fallback handler for 404 errors
async fn fallback_handler() -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "404: Not Found")
}