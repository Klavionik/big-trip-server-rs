use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
struct Offer {
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

#[derive(Serialize, Deserialize, FromRow)]
struct Picture {
    description: String,
    src: String,
}

#[derive(Serialize, FromRow)]
pub struct Destination {
    id: Uuid,
    name: String,
    description: String,
    pictures: sqlx::types::Json<Vec<Picture>>,
}

#[derive(Deserialize)]
pub struct EventCreate {
    #[serde(rename = "type")]
    pub kind: String,
    pub destination: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub date_from: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub date_to: OffsetDateTime,
    pub offers: Vec<Uuid>,
    pub base_price: i64,
    pub is_favorite: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: Uuid,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub kind: String,
    pub destination: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub date_from: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub date_to: OffsetDateTime,
    pub offers: sqlx::types::Json<Vec<Uuid>>,
    pub base_price: i64,
    pub is_favorite: bool,
}

#[derive(Serialize)]
pub struct SyncResult {
    pub updated: Vec<Event>,
}
