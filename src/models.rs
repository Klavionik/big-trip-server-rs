use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Offer {
    id: Uuid,
    title: String,
    price: i64,
}

#[derive(Serialize, FromRow)]
pub struct Activity {
    #[sqlx(rename = "type")]
    #[serde(rename(serialize = "type"))]
    kind: String,
    offers: sqlx::types::Json<Vec<Offer>>,
}

#[derive(Serialize)]
struct Picture {
    description: String,
    src: String,
}

#[derive(Serialize)]
struct Destination {
    id: Uuid,
    name: String,
    description: String,
    pictures: Vec<Picture>,
}

#[derive(Deserialize)]
struct EventCreate {
    kind: String,
    destination: Uuid,
    date_from: OffsetDateTime,
    date_to: OffsetDateTime,
    offers: Vec<Uuid>,
    base_price: i64,
    is_favorite: bool,
}

#[derive(Serialize)]
struct Event {
    id: Uuid,
    kind: String,
    destination: Uuid,
    date_from: OffsetDateTime,
    date_to: OffsetDateTime,
    offers: Vec<Uuid>,
    base_price: i64,
    is_favorite: bool,
}

#[derive(Serialize)]
struct SyncResult {
    updated: Vec<Event>,
}
