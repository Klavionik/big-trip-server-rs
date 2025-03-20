use crate::models::{Activity, Destination, Event, EventCreate};
use sqlx::PgPool;
use uuid::Uuid;

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

pub async fn create_event(event: EventCreate, pool: &PgPool) -> Event {
    let new_id: Uuid = sqlx::query_scalar(
        "INSERT INTO events (type, destination, date_from, date_to, offers, base_price, is_favorite)
            VALUES ($1, $2, $3, $4, $5::jsonb, $6, $7)
            RETURNING id
            "
    )
        .bind(&event.kind)
        .bind(event.destination)
        .bind(event.date_from)
        .bind(event.date_to)
        .bind(serde_json::to_string(&event.offers).unwrap())
        .bind(event.base_price)
        .bind(event.is_favorite)
        .fetch_one(pool)
        .await
        .unwrap();

    Event {
        id: new_id,
        kind: event.kind,
        destination: event.destination,
        date_from: event.date_from,
        date_to: event.date_to,
        offers: sqlx::types::Json(event.offers),
        base_price: event.base_price,
        is_favorite: event.is_favorite,
    }
}
