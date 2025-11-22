use sqlx::PgPool;
use uuid::Uuid;
use crate::handlers::users::NewUserRequest;
use crate::utils::password;

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<crate::models::user::User>, sqlx::Error> {
    let users = sqlx::query_as!(
        crate::models::user::User,
        "SELECT * FROM core.users"
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

pub async fn create_new_user(pool: &PgPool, payload: NewUserRequest) -> Result<(), sqlx::Error> {
    let password_hash = password::hash_password(&payload.password)?;

    sqlx::query!(
        r#"
        INSERT INTO core.users (id, full_name, email, password_hash, department_id, role_id)
        VALUES ($1,$2,$3,$4,$5,$6)
        "#,
        Uuid::new_v4(),
        payload.full_name,
        payload.email,
        password_hash,
        payload.department_id,
        payload.role_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
