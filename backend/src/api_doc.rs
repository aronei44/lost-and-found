use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handler::auth_handler::register,
        crate::handler::auth_handler::login,
        crate::handler::auth_handler::refresh_token
    ),
    components(schemas(
        crate::model::user_model::RegisterRequest, 
        crate::model::user_model::LoginRequest, 
        crate::model::user_model::TokenResponse, 
        crate::model::user_model::RefreshRequest
    )),
    tags(
        (name = "Auth", description = "Authentication endpoints")
    ),
)]
pub struct ApiDoc;