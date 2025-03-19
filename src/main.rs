mod crud;
mod models;
mod settings;

use crate::settings::Settings;
use actix_web::{web, App, HttpServer, Responder};
use config::{Config, Environment};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::io::Result;

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

// async fn create_event() -> Result<impl Responder> {}

// async fn update_event() -> Result<impl Responder> {}

// async fn delete_event() -> Result<impl Responder> {}

async fn get_destinations(state: web::Data<AppState>) -> Result<impl Responder> {
    let destinations = crud::get_destinations(&state.db).await;

    Ok(web::Json(destinations))
}

// async fn sync_events() -> Result<impl Responder> {}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::builder()
        .set_default("database_url", "postgres://user:password@db:5432/bigtrip")
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
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .route("/healthz", web::get().to(healthz))
            .route("/activities", web::get().to(get_activities))
            .route("/events", web::get().to(get_events))
            // .route("/points/{event_id}", web::post().to(create_event))
            // .route("/points/{event_id}", web::put().to(update_event))
            // .route("/points/{event_id}", web::delete().to(delete_event))
            .route("/destinations", web::get().to(get_destinations))
        // .route("/points/sync", web::post().to(sync_events))
    })
    .workers(1)
    .bind(("0.0.0.0", 9336))?
    .run()
    .await
}
