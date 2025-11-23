use sqlx::PgPool;
use crate::handlers::task::NewTaskRequest;

// pub async fn get_all_tasks(pool: &PgPool) -> Result<Vec<crate::models::task::Task>, sqlx::Error> {
//     let tasks = sqlx::query_as!(
//         crate::models::task::Task,
//         "SELECT * FROM core.task"
//     )
//     .fetch_all(pool)
//     .await?;
//     Ok(tasks)
// }

pub async fn create_task(pool: &PgPool, payload: NewTaskRequest) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO core.tasks (title, description)
        VALUES ($1,$2)
        "#,
        payload.title,
        payload.description
    )
    .execute(pool)
    .await?;
    Ok(())
}
