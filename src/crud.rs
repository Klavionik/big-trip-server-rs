use crate::models::Activity;
use sqlx::PgPool;

pub async fn get_activities(pool: &PgPool) -> Vec<Activity> {
    sqlx::query_as("SELECT * FROM activities;")
        .fetch_all(pool)
        .await
        .unwrap()
}
