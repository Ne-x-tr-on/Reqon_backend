use sqlx::PgPool;
use crate::handlers::auth::{RegisterRequest, LoginRequest};
use crate::utils::password;
use crate::utils::jwt;
use crate::models::user::User;

pub async fn register_user(pool: &PgPool, payload: RegisterRequest) -> Result<(), sqlx::Error> {
    let password_hash = password::hash_password(&payload.password)?;

    sqlx::query!(
        r#"
        INSERT INTO core.users (full_name, email, password_hash, phone, employee_number, dob, gender, department_id, role_id)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
        "#,
        payload.full_name,
        payload.email,
        password_hash,
        payload.phone,
        uuid::Uuid::new_v4().to_string(),
        payload.dob,
        payload.gender,
        payload.department_id,
        payload.role_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn login_user(pool: &PgPool, payload: LoginRequest) -> Result<String, sqlx::Error> {
    let user: Option<User> = sqlx::query_as!(
        User,
        r#"SELECT * FROM core.users WHERE email = $1"#,
        payload.email
    )
    .fetch_optional(pool)
    .await?;

    match user {
        Some(u) if password::verify_password(&payload.password, &u.password_hash)? => {
            let token = jwt::create_jwt(u.id)?;
            Ok(token)
        }
        _ => Err(sqlx::Error::RowNotFound),
    }
}
