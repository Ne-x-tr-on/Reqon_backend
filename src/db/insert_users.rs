use crate::models::user::{NewUser, User};
use crate::utils::password::hash_password;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::NaiveDate;

/// Insert a new user into the database
pub async fn create_user(pool: &PgPool, new_user: NewUser) -> Result<User, sqlx::Error> {
    // Hash the password
    let hashed_password = hash_password(&new_user.password)
        .map_err(|_| sqlx::Error::Protocol("Password hashing failed".into()))?;

    // Parse DOB string into NaiveDate
    let dob = NaiveDate::parse_from_str(&new_user.dob, "%Y-%m-%d")
        .map_err(|_| sqlx::Error::Protocol("Invalid date format for DOB".into()))?;

    // Insert user into the database
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO core.users
        (id, full_name, email, password_hash, phone, employee_number, dob, gender, department_id, role_id, created_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,NOW())
        RETURNING id, full_name, email, password_hash, phone, employee_number, dob, gender, department_id, role_id, created_at
        "#,
        Uuid::new_v4(),              // id
        new_user.full_name,          // full_name
        new_user.email,              // email
        hashed_password,             // password_hash
        new_user.phone,              // phone (optional)
        new_user.employee_number,    // employee_number
        dob,                         // dob
        new_user.gender,             // gender (optional)
        new_user.department_id,      // department_id
        new_user.role_id             // role_id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
