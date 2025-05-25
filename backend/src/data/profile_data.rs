use crate::helper::db::create_pool;
use crate::model::profile_model::{Profile, UpdateProfileRequest};


pub async fn get_profile_by_username(username: &str) -> sqlx::Result<Profile> {
    let pool = create_pool().await?;
    let user = sqlx::query_as!(
        Profile,
        r#"
        SELECT username, full_name, email, phone, address
        FROM profiles
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(&pool)
    .await?;
    Ok(user)
}

pub async fn update_profile(data: &UpdateProfileRequest) -> sqlx::Result<Profile> {
    let pool = create_pool().await?;
    // Hash the password before storing it
    let user = sqlx::query_as!(
        Profile,
        r#"
        UPDATE profiles
        SET full_name = $2, email = $3, phone = $4, address = $5
        WHERE username = $1
        RETURNING username, full_name, email, phone, address
        "#,
        data.username,
        data.full_name,
        data.email,
        data.phone,
        data.address
    )
    .fetch_one(&pool)
    .await?;
    Ok(user)
}

pub async fn create_profile(data: &UpdateProfileRequest) -> sqlx::Result<Profile> {
    let pool = create_pool().await?;
    // Hash the password before storing it
    let user = sqlx::query_as!(
        Profile,
        r#"
        INSERT INTO profiles (username, full_name, email, phone, address)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING username, full_name, email, phone, address
        "#,
        data.username,
        data.full_name,
        data.email,
        data.phone,
        data.address
    )
    .fetch_one(&pool)
    .await?;
    Ok(user)
}