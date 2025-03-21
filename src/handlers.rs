use crate::crud;
use crate::models::{Event, EventCreate};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub struct AppState {
    pub db: PgPool,
}

pub async fn healthz() -> &'static str {
    "OK"
}

pub async fn get_activities(state: web::Data<AppState>) -> std::io::Result<impl Responder> {
    let activities = crud::get_activities(&state.db).await;

    Ok(web::Json(activities))
}

pub async fn get_events(state: web::Data<AppState>) -> std::io::Result<impl Responder> {
    let events = crud::get_events(&state.db).await;

    Ok(web::Json(events))
}

pub async fn create_event(
    event: web::Json<EventCreate>,
    state: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let new_event = crud::create_event(event.into_inner(), &state.db).await;

    Ok(web::Json(new_event))
}

pub async fn update_event(
    event_id: web::Path<(Uuid,)>,
    event: web::Json<Event>,
    state: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let event_id = event_id.into_inner().0;
    let event = event.into_inner();
    let event = crud::update_event(event_id, event, &state.db).await;

    Ok(web::Json(event))
}

pub async fn delete_event(
    event_id: web::Path<(Uuid,)>,
    state: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let event_id = event_id.into_inner().0;
    crud::delete_event(event_id, &state.db).await;

    Ok(HttpResponse::NoContent())
}

pub async fn get_destinations(state: web::Data<AppState>) -> std::io::Result<impl Responder> {
    let destinations = crud::get_destinations(&state.db).await;

    Ok(web::Json(destinations))
}

pub async fn sync_events(
    events: web::Json<Vec<Event>>,
    state: web::Data<AppState>,
) -> std::io::Result<impl Responder> {
    let events = events.into_inner();
    let result = crud::sync_events(events, &state.db).await;

    Ok(web::Json(result))
}
