use crate::helper::db::create_pool;
use crate::model::photo_model::{CreatePhoto, Photo, PhotoWithLostPeople};
use crate::model::photo_data_model::{CreatePhotoData, PhotoData};
use crate::model::place_model::Place;

pub async fn get_photos_by_person_id(person_id: i32) -> sqlx::Result<Vec<Photo>> {
    let pool = create_pool().await?;
    let photos = sqlx::query_as!(
        Photo,
        r#"
        select photos.id, path, bucket from photos
        join photo_data pd on pd.photo_id = photos.id
        where pd.lost_people_id = $1 and pd.place_id is null
        "#,
        person_id
    )
    .fetch_all(&pool)
    .await?;
    Ok(photos)
}

pub async fn get_places_by_person_id(person_id: i32) -> sqlx::Result<Vec<Place>> {
    let pool = create_pool().await?;
    let places = sqlx::query_as!(
        Place,
        r#"
        select p.id, p.user_username, p.latitude, p.longitude, p.created_at
        from places p
        join photo_data pd on pd.place_id = p.id
        where pd.lost_people_id = $1
        "#,
        person_id
    )
    .fetch_all(&pool)
    .await?;
    Ok(places)
}

pub async fn get_photos_by_person_id_and_place_id(
    person_id: i32,
    place_id: i32,
) -> sqlx::Result<Vec<Photo>> {
    let pool = create_pool().await?;
    let photos = sqlx::query_as!(
        Photo,
        r#"
        select photos.id, path, bucket from photos
        join photo_data pd on pd.photo_id = photos.id
        where pd.lost_people_id = $1 and pd.place_id = $2
        "#,
        person_id,
        place_id
    )
    .fetch_all(&pool)
    .await?;
    Ok(photos)
}


pub async fn get_photos_with_lost_by_username(username: &str) -> sqlx::Result<Vec<PhotoWithLostPeople>> {
    let pool = create_pool().await?;
    let photos = sqlx::query_as!(
        PhotoWithLostPeople,
        r#"
        select 
            p.id, path, bucket,
            lp.id as person_id,
            lp.fullname,
            lp.alias,
            lp.gender,
            lp.born_date,
            lp.last_condition,
            lp.is_found,
            lp.lost_date,
            lp.found_date
        from photos p
        join photo_data pd on pd.photo_id = p.id
        join lost_people lp on lp.id = pd.lost_people_id
        join places ps on ps.id = pd.place_id
        where ps.user_username = $1
        "#,
        username
    )
    .fetch_all(&pool)
    .await?;
    Ok(photos)
}

pub async fn create_photo(photo: CreatePhoto) -> Result<Photo, Box<dyn std::error::Error + Send + Sync>> {
    let pool = create_pool().await?;
    let new_photo = sqlx::query_as!(
        Photo,
        r#"
        insert into photos (bucket, path)
        values ($1, $2)
        returning id, bucket, path
        "#,
        photo.bucket,
        photo.path
    )
    .fetch_one(&pool)
    .await?;
    Ok(new_photo)
}

pub async fn create_photo_data(photo_data: CreatePhotoData) -> Result<PhotoData, Box<dyn std::error::Error + Send + Sync>> {
    let pool = create_pool().await?;
    let new_photo_data = sqlx::query_as!(
        PhotoData,
        r#"
        insert into photo_data (photo_id, lost_people_id, place_id)
        values ($1, $2, $3)
        returning id, photo_id, lost_people_id, place_id
        "#,
        photo_data.photo_id,
        photo_data.lost_people_id,
        photo_data.place_id
    )
    .fetch_one(&pool)
    .await?;
    Ok(new_photo_data)
}