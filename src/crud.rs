use crate::models::{Activity, Destination, Event, EventCreate, SyncResult};
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum CRUDError {
    #[error("Operation failed.")]
    UnknownError,
    #[error("Destination with ID {0} doesn't exist.")]
    IncorrectDestination(Uuid),
}

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

pub async fn create_event(event: EventCreate, pool: &PgPool) -> Result<Event, CRUDError> {
    let query = sqlx::query_scalar(
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
        .bind(event.is_favorite);

    let new_id = query.fetch_one(pool).await.map_err(|sqlx_error| {
        if let Some(db_error) = sqlx_error.as_database_error() {
            if db_error.is_foreign_key_violation() {
                return CRUDError::IncorrectDestination(event.destination);
            }
        }

        CRUDError::UnknownError
    })?;

    Ok(Event {
        id: new_id,
        kind: event.kind,
        destination: event.destination,
        date_from: event.date_from,
        date_to: event.date_to,
        offers: sqlx::types::Json(event.offers),
        base_price: event.base_price,
        is_favorite: event.is_favorite,
    })
}

pub async fn update_event(event_id: Uuid, event: Event, pool: &PgPool) -> Event {
    sqlx::query("UPDATE events SET type = $2, destination = $3, date_from = $4, date_to = $5, offers = $6::jsonb, base_price = $7, is_favorite = $8 WHERE id = $1")
        .bind(event_id)
        .bind(&event.kind)
        .bind(event.destination)
        .bind(event.date_from)
        .bind(event.date_to)
        .bind(serde_json::to_string(&event.offers).unwrap())
        .bind(event.base_price)
        .bind(event.is_favorite)
        .execute(pool)
        .await
        .unwrap();

    event
}

pub async fn delete_event(event_id: Uuid, pool: &PgPool) {
    sqlx::query("DELETE FROM events WHERE id = $1")
        .bind(event_id)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn sync_events(events: Vec<Event>, pool: &PgPool) -> SyncResult {
    let mut updated = vec![];

    for event in events {
        let event = update_event(event.id, event, pool).await;
        updated.push(event)
    }

    SyncResult { updated }
}
