use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::{User, NewUser};
use crate::utils::password;
use chrono::{NaiveDate, Utc};

/// Get all users
pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, full_name, email, password_hash, phone, employee_number, dob, gender, department_id, role_id, created_at
         FROM core.users"
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

/// Create a new user
pub async fn create_new_user(pool: &PgPool, payload: NewUser) -> Result<(), sqlx::Error> {
    // Hash password
    let password_hash = password::hash_password(&payload.password)
        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

    // Parse DOB string to NaiveDate
    let dob: NaiveDate = NaiveDate::parse_from_str(&payload.dob, "%Y-%m-%d")
        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

    // Insert into DB
    sqlx::query(
        "INSERT INTO core.users
        (id, full_name, email, password_hash, phone, employee_number, dob, gender, department_id, role_id, created_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)"
    )
    .bind(Uuid::new_v4())
    .bind(&payload.full_name)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.phone)
    .bind(&payload.employee_number)
    .bind(dob)
    .bind(&payload.gender)
    .bind(payload.department_id) // if nullable, make it Option<i16>
    .bind(payload.role_id)       // if nullable, make it Option<i16>
    .bind(Utc::now().naive_utc()) // created_at
    .execute(pool)
    .await?;

    Ok(())
}
