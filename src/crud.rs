use crate::models::{Activity, Destination, Event, EventCreate, SyncResult};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum CRUDError {
    #[error("Operation failed.")]
    UnknownError(#[from] sqlx::Error),
    #[error("Destination with ID {0} doesn't exist.")]
    IncorrectDestination(Uuid),
}

pub async fn get_activities(pool: &PgPool) -> Result<Vec<Activity>, CRUDError> {
    let activities: Vec<Activity> = sqlx::query_as("SELECT * FROM activities;")
        .fetch_all(pool)
        .await?;

    Ok(activities)
}

pub async fn get_events(pool: &PgPool) -> Result<Vec<Event>, CRUDError> {
    let events: Vec<Event> = sqlx::query_as("SELECT * FROM events;")
        .fetch_all(pool)
        .await?;

    Ok(events)
}

pub async fn get_destinations(pool: &PgPool) -> Result<Vec<Destination>, CRUDError> {
    let destinations = sqlx::query_as("SELECT * FROM destinations;")
        .fetch_all(pool)
        .await?;

    Ok(destinations)
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

        CRUDError::UnknownError(sqlx_error)
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

pub async fn update_event(event_id: Uuid, event: Event, pool: &PgPool) -> Result<Event, CRUDError> {
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
        .await?;

    Ok(event)
}

pub async fn delete_event(event_id: Uuid, pool: &PgPool) -> Result<(), CRUDError> {
    let _ = sqlx::query("DELETE FROM events WHERE id = $1")
        .bind(event_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn sync_events(events: Vec<Event>, pool: &PgPool) -> Result<SyncResult, CRUDError> {
    let mut updated = vec![];

    for event in events {
        let event = update_event(event.id, event, pool).await?;
        updated.push(event)
    }

    Ok(SyncResult { updated })
}
