use serde::{Serialize, Deserialize};
use utoipa::{
    ToSchema,
    OpenApi
};


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TokenResponse {
    pub access_token: String,
}


#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handler::auth_handler::register,
        crate::handler::auth_handler::login
    ),
    components(schemas(RegisterRequest, LoginRequest, TokenResponse)),
    tags(
        (name = "Auth", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;