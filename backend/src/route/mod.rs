use axum::{Router, routing::{post, put, get}, middleware};
use crate::{
    handler::{
        auth_handler::{register, login, refresh_token},
        profile_handler::{
            update_profile_handler,
            get_profile_handler
        },
        lost_people_handler::{
            get_monitored_people_handler,
            get_person_by_id_handler,
            create_person_handler,
            update_person_handler,
            add_monitoring_handler,
            get_all_lost_people_handler,
            get_founder_by_username_handler
        },
        data_handler::{
            get_photos_by_person_id_handler,
            get_places_by_person_id_handler,
            get_photos_by_person_id_and_place_id_handler,
            get_photos_with_lost_by_username_handler,
            upload_photo_handler,
            recognize_target_handler
        }
    },
    helper::auth::authorization_middleware
};

pub fn routes() -> Router {
    Router::new()
    .nest("/auth", 
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route(
            "/refresh",
            post(refresh_token)
        )
    )
    .nest("/profile", Router::new()
        .route("/", put(update_profile_handler))
        .route("/", get(get_profile_handler))
        .layer(middleware::from_fn(authorization_middleware))
    )
    .nest("/lost_people", 
        Router::new()
            .route("/", get(get_monitored_people_handler))
            .route("/{id}", get(get_person_by_id_handler))
            .route("/", post(create_person_handler))
            .route("/", put(update_person_handler))
            .route("/{id}", post(add_monitoring_handler))
            .route("/all", get(get_all_lost_people_handler))
            .layer(middleware::from_fn(authorization_middleware))
    )
    .nest("/data", 
        Router::new()
            .route("/photos/{person_id}", get(get_photos_by_person_id_handler))
            .route("/places/{person_id}", get(get_places_by_person_id_handler))
            .route("/photos/{person_id}/{place_id}", get(get_photos_by_person_id_and_place_id_handler))
            .route("/monitoring", get(get_photos_with_lost_by_username_handler))
            .route("/upload_photo/{person_id}", post(upload_photo_handler))
            .route("/recognize_target", post(recognize_target_handler))
            .route("/founder/{current_user}", get(get_founder_by_username_handler))
            .layer(middleware::from_fn(authorization_middleware))
    )
}