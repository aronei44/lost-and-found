use crate::helper::db::create_pool;
use crate::model::user_model::User;
use bcrypt;


pub async fn get_user_by_username(username: &str) -> sqlx::Result<User> {
    let pool = create_pool().await?;
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, password, created_at, last_active
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(&pool)
    .await?;
    Ok(user)
}

pub async fn create_user(username: &str, password: &str) -> sqlx::Result<User> {
    let pool = create_pool().await?;
    // Hash the password before storing it
    let hashed_password = hash_password(password).await?;
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password)
        VALUES ($1, $2)
        RETURNING username, password, created_at, last_active
        "#,
        username,
        hashed_password
    )
    .fetch_one(&pool)
    .await?;
    Ok(user)
}

pub async fn update_user_last_active(username: &str) -> sqlx::Result<()> {
    let pool = create_pool().await?;
    sqlx::query!(
        r#"
        UPDATE users
        SET last_active = NOW()
        WHERE username = $1
        "#,
        username
    )
    .execute(&pool)
    .await?;
    Ok(())
}

pub async fn hash_password(password: &str) -> sqlx::Result<String> {
    // Use a hashing library like bcrypt or argon2 to hash the password
    // For example, using bcrypt:
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    Ok(hashed_password)
}

pub async fn verify_password(password: &str, hashed_password: &str) -> sqlx::Result<bool> {
    // Use a hashing library like bcrypt or argon2 to verify the password
    // For example, using bcrypt:
    let is_valid = bcrypt::verify(password, hashed_password)
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    Ok(is_valid)
}
