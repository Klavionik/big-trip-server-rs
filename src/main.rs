mod crud;
mod models;
mod settings;

use crate::models::{Event, EventCreate};
use crate::settings::Settings;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpResponse, HttpServer, Responder};
use config::{Config, Environment};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::io::Result;
use uuid::Uuid;

struct AppState {
    db: PgPool,
}

async fn get_activities(state: web::Data<AppState>) -> Result<impl Responder> {
    let activities = crud::get_activities(&state.db).await;

    Ok(web::Json(activities))
}

async fn healthz() -> &'static str {
    "OK"
}

async fn get_events(state: web::Data<AppState>) -> Result<impl Responder> {
    let events = crud::get_events(&state.db).await;

    Ok(web::Json(events))
}

async fn create_event(
    event: web::Json<EventCreate>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let new_event = crud::create_event(event.into_inner(), &state.db).await;

    Ok(web::Json(new_event))
}

async fn update_event(
    event_id: web::Path<(Uuid,)>,
    event: web::Json<Event>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let event_id = event_id.into_inner().0;
    let event = event.into_inner();
    let event = crud::update_event(event_id, event, &state.db).await;

    Ok(web::Json(event))
}

async fn delete_event(
    event_id: web::Path<(Uuid,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let event_id = event_id.into_inner().0;
    crud::delete_event(event_id, &state.db).await;

    Ok(HttpResponse::NoContent())
}

async fn get_destinations(state: web::Data<AppState>) -> Result<impl Responder> {
    let destinations = crud::get_destinations(&state.db).await;

    Ok(web::Json(destinations))
}

async fn sync_events(
    events: web::Json<Vec<Event>>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let events = events.into_inner();
    let result = crud::sync_events(events, &state.db).await;

    Ok(web::Json(result))
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::builder()
        .set_default("database_url", "postgres://user:password@db:5432/bigtrip")
        .unwrap()
        .set_default("allowed_origin", "http://localhost:8080")
        .unwrap()
        .add_source(Environment::default())
        .build()
        .unwrap();
    let settings = config.try_deserialize::<Settings>().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.database_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&settings.allowed_origin)
            .allowed_methods(vec![
                http::Method::GET,
                http::Method::POST,
                http::Method::PUT,
                http::Method::DELETE,
            ])
            .allowed_header(http::header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .route("/healthz", web::get().to(healthz))
            .route("/activities", web::get().to(get_activities))
            .route("/events", web::get().to(get_events))
            .route("/events", web::post().to(create_event))
            .route("/events/{event_id}", web::put().to(update_event))
            .route("/events/{event_id}", web::delete().to(delete_event))
            .route("/destinations", web::get().to(get_destinations))
            .route("/events/sync", web::post().to(sync_events))
    })
    .workers(1)
    .bind(("0.0.0.0", 9336))?
    .run()
    .await
}
