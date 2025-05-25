use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{SecurityScheme, HttpAuthScheme, HttpBuilder};

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
        (name = "Lost And Found", description = "gitulah")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;


struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut components = openapi.components.clone().unwrap_or_default();
        components.security_schemes.insert(
            "bearer_auth".to_string(),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        openapi.components = Some(components);
    }
}