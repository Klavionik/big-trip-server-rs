use crate::crud;
use crate::models::{Event, EventCreate};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
struct APIError {
    detail: String,
}

impl ResponseError for crud::CRUDError {
    fn status_code(&self) -> StatusCode {
        match *self {
            crud::CRUDError::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            crud::CRUDError::IncorrectDestination(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(APIError {
            detail: self.to_string(),
        })
    }
}

pub async fn healthz() -> &'static str {
    "OK"
}

pub async fn get_activities(db: web::Data<PgPool>) -> actix_web::Result<impl Responder> {
    let activities = crud::get_activities(&db).await?;

    Ok(web::Json(activities))
}

pub async fn get_events(db: web::Data<PgPool>) -> actix_web::Result<impl Responder> {
    let events = crud::get_events(&db).await?;

    Ok(web::Json(events))
}

pub async fn create_event(
    event: web::Json<EventCreate>,
    db: web::Data<PgPool>,
) -> actix_web::Result<impl Responder> {
    let new_event = crud::create_event(event.into_inner(), &db).await?;

    Ok(HttpResponse::Created().json(new_event))
}

pub async fn update_event(
    event_id: web::Path<(Uuid,)>,
    event: web::Json<Event>,
    db: web::Data<PgPool>,
) -> actix_web::Result<impl Responder> {
    let event_id = event_id.into_inner().0;
    let event = event.into_inner();
    let event = crud::update_event(event_id, event, &db).await?;

    Ok(web::Json(event))
}

pub async fn delete_event(
    event_id: web::Path<(Uuid,)>,
    db: web::Data<PgPool>,
) -> actix_web::Result<impl Responder> {
    let event_id = event_id.into_inner().0;
    crud::delete_event(event_id, &db).await?;

    Ok(HttpResponse::NoContent())
}

pub async fn get_destinations(db: web::Data<PgPool>) -> actix_web::Result<impl Responder> {
    let destinations = crud::get_destinations(&db).await?;

    Ok(web::Json(destinations))
}

pub async fn sync_events(
    events: web::Json<Vec<Event>>,
    db: web::Data<PgPool>,
) -> actix_web::Result<impl Responder> {
    let events = events.into_inner();
    let result = crud::sync_events(events, &db).await?;

    Ok(web::Json(result))
}
