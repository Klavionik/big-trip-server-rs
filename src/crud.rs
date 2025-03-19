use crate::models::{Activity, Destination, Event};
use sqlx::PgPool;

pub async fn get_activities(pool: &PgPool) -> Vec<Activity> {
    sqlx::query_as("SELECT * FROM activities;")
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn get_events(pool: &PgPool) -> Vec<Event> {
    sqlx::query_as("SELECT * FROM events;")
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn get_destinations(pool: &PgPool) -> Vec<Destination> {
    sqlx::query_as("SELECT * FROM destinations;")
        .fetch_all(pool)
        .await
        .unwrap()
}
