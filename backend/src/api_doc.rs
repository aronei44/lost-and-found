use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{SecurityScheme, HttpAuthScheme, HttpBuilder};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handler::auth_handler::register,
        crate::handler::auth_handler::login,
        crate::handler::auth_handler::refresh_token,
        crate::handler::profile_handler::update_profile_handler,
        crate::handler::profile_handler::get_profile_handler,
        crate::handler::lost_people_handler::get_monitored_people_handler,
        crate::handler::lost_people_handler::get_person_by_id_handler,
        crate::handler::lost_people_handler::create_person_handler,
        crate::handler::lost_people_handler::update_person_handler,
        crate::handler::lost_people_handler::add_monitoring_handler,
        crate::handler::lost_people_handler::get_all_lost_people_handler,
        crate::handler::data_handler::get_photos_by_person_id_handler,
        crate::handler::data_handler::get_places_by_person_id_handler,
        crate::handler::data_handler::get_photos_by_person_id_and_place_id_handler,
        crate::handler::data_handler::get_photos_with_lost_by_username_handler,
        crate::handler::data_handler::upload_photo_handler,
        crate::handler::data_handler::recognize_target_handler
    ),
    components(schemas(
        crate::model::user_model::RegisterRequest, 
        crate::model::user_model::LoginRequest, 
        crate::model::user_model::TokenResponse, 
        crate::model::user_model::RefreshRequest,
        crate::model::profile_model::ProfileResponse,
        crate::model::profile_model::UpdateProfileRequest,
        crate::model::lost_people_model::LostPeople,
        crate::model::lost_people_model::CreateLostPeopleRequest,
        crate::model::lost_people_model::UpdateLostPeopleRequest,
        crate::model::photo_model::Photo,
        crate::model::photo_model::CreatePhoto,
        crate::model::photo_model::PhotoWithLostPeople,
        crate::model::photo_data_model::PhotoData,
        crate::model::photo_data_model::CreatePhotoData,
        crate::model::place_model::Place,
        crate::model::place_model::CreatePlace
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