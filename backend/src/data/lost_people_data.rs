use crate::helper::db::create_pool;
use crate::model::lost_people_model::{LostPeople, UpdateLostPeopleRequest, CreateLostPeopleRequest};

pub async fn get_monitored_people(username: &str) -> sqlx::Result<Vec<LostPeople>> {
    let pool = create_pool().await?;
    let users = sqlx::query_as!(
        LostPeople,
        r#"
        select 
            id, fullname, alias, gender, born_date, last_condition, is_found, lost_date, found_date
        from lost_people lp
        JOIN monitor_people mp on mp.lost_people_id = lp.id and mp.user_username = $1
        order by lp.lost_date desc
        "#,
        username
    )
    .fetch_all(&pool)
    .await?;
    Ok(users)
}

pub async fn get_person_by_id(id: i32) -> sqlx::Result<Option<LostPeople>> {
    let pool = create_pool().await?;
    let person = sqlx::query_as!(
        LostPeople,
        r#"
        select 
            id, fullname, alias, gender, born_date, last_condition, is_found, lost_date, found_date
        from lost_people
        where id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?;
    Ok(person)
}


pub async fn create_person(
    person: CreateLostPeopleRequest,
    username: &str,
) -> sqlx::Result<LostPeople> {
    let pool = create_pool().await?;
    let new_person = sqlx::query_as!(
        LostPeople,
        r#"
        insert into lost_people (fullname, alias, gender, born_date, last_condition, lost_date)
        values ($1, $2, $3, $4, $5, $6)
        returning id, fullname, alias, gender, born_date, last_condition, is_found, lost_date, found_date
        "#,
        person.fullname,
        person.alias,
        person.gender,
        person.born_date,
        person.last_condition,
        person.lost_date
    )
    .fetch_one(&pool)
    .await?;
    sqlx::query!(
        r#"
        insert into monitor_people (user_username, lost_people_id)
        values ($1, $2)
        "#,
        username,
        new_person.id
    )
    .execute(&pool)
    .await?;
    Ok(new_person)
}


pub async fn update_person(
    person: UpdateLostPeopleRequest,
    username: &str,
) -> sqlx::Result<Option<LostPeople>> {
    let pool = create_pool().await?;
    let updated_person = sqlx::query_as!(
        LostPeople,
        r#"
        update lost_people
        set fullname = coalesce($1, fullname),
            alias = coalesce($2, alias),
            gender = coalesce($3, gender),
            born_date = coalesce($4, born_date),
            last_condition = coalesce($5, last_condition),
            is_found = coalesce($6, is_found),
            lost_date = coalesce($7, lost_date),
            found_date = coalesce($8, found_date)
        from monitor_people mp
        where lost_people.id = $9
          and mp.lost_people_id = lost_people.id
          and mp.user_username = $10
        returning lost_people.id, fullname, alias, gender, born_date, last_condition, is_found, lost_date, found_date
        "#,
        person.fullname,
        person.alias,
        person.gender,
        person.born_date,
        person.last_condition,
        person.is_found,
        person.lost_date,
        person.found_date,
        person.id,
        username
    )
    .fetch_optional(&pool)
    .await?;
    Ok(updated_person)
}

pub async fn add_monitoring(
    person_id: i32,
    username: &str,
) -> sqlx::Result<()> {
    let pool = create_pool().await?;
    sqlx::query!(
        r#"
        insert into monitor_people (user_username, lost_people_id)
        values ($1, $2)
        "#,
        username,
        person_id
    )
    .execute(&pool)
    .await?;
    Ok(())
}

pub async fn all_lost_people() -> sqlx::Result<Vec<LostPeople>> {
    let pool = create_pool().await?;
    let people = sqlx::query_as!(
        LostPeople,
        r#"
        select 
            id, fullname, gender, alias, born_date, last_condition, is_found, lost_date, found_date
        from lost_people
        where is_found = false
        order by lost_date desc
        "#,
    )
    .fetch_all(&pool)
    .await?;
    Ok(people)
}