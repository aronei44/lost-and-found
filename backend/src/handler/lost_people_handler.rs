use axum::{
    Json, 
    http::StatusCode
};
use crate::model::lost_people_model::{UpdateLostPeopleRequest, CreateLostPeopleRequest, LostPeople};
use crate::data::lost_people_data::{add_monitoring, get_monitored_people, get_person_by_id, create_person};
use axum::extract::Extension;
use crate::model::user_model::User;



#[utoipa::path(
    get,
    path = "/api/lost_people",
    responses(
        (status = 200, description = "List of Monitored People", body = Vec<LostPeople>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_monitored_people_handler(Extension(current_user): Extension<User>) -> Result<Json<Vec<LostPeople>>, (StatusCode, Json<serde_json::Value>)> {
    let people = get_monitored_people(&current_user.username).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;
    Ok(Json(people))
}


#[utoipa::path(
    get,
    path = "/api/lost_people/{id}",
    params(
        ("id" = i32, Path, description = "ID of the person")
    ),
    responses(
        (status = 200, description = "Person Found", body = LostPeople),
        (status = 404, description = "Person Not Found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]

pub async fn get_person_by_id_handler(
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Json<LostPeople>, (StatusCode, Json<serde_json::Value>)> {
    match get_person_by_id(id).await {
        Ok(Some(person)) => Ok(Json(person)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Person not found"})),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/lost_people",
    request_body = CreateLostPeopleRequest,
    responses(
        (status = 201, description = "Person Created", body = LostPeople)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_person_handler(
    Extension(current_user): Extension<User>,
    Json(payload): Json<CreateLostPeopleRequest>,
) -> Result<Json<LostPeople>, (StatusCode, Json<serde_json::Value>)> {
    let person = create_person(payload, &current_user.username).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;
    Ok(Json(person))
}

#[utoipa::path(
    put,
    path = "/api/lost_people",
    request_body = UpdateLostPeopleRequest,
    responses(
        (status = 200, description = "Person Updated", body = LostPeople),
        (status = 404, description = "Person Not Found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_person_handler(
    Extension(current_user): Extension<User>,
    Json(payload): Json<UpdateLostPeopleRequest>,
) -> Result<Json<LostPeople>, (StatusCode, Json<serde_json::Value>)> {
    match get_person_by_id(payload.id).await {
        Ok(Some(_)) => {
            let updated_person = crate::data::lost_people_data::update_person(payload, &current_user.username).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
            })?;
            match updated_person {
                Some(person) => Ok(Json(person)),
                None => Err((
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"error": "Person not found"})),
                )),
            }
        },
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Person not found"})),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )),
    }
}


#[utoipa::path(
    post,
    path = "/api/lost_people/{id}",
    params(
        ("id" = i32, Path, description = "ID of the person")
    ),
    responses(
        (status = 200, description = "Monitored Person Added"),
        (status = 404, description = "Person Not Found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn add_monitoring_handler(
    Extension(current_user): Extension<User>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    match add_monitoring(id, &current_user.username).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )),
    }
}


#[utoipa::path(
    get,
    path = "/api/lost_people/all",
    responses(
        (status = 200, description = "All lost people retrieved", body = Vec<LostPeople>),
        (status = 404, description = "Person Not Found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_all_lost_people_handler() -> Result<Json<Vec<LostPeople>>, (StatusCode, Json<serde_json::Value>)> {
    match crate::data::lost_people_data::all_lost_people().await {
        Ok(people) => Ok(Json(people)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )),
    }
}