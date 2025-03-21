mod crud;
mod handlers;
mod models;
mod settings;

use crate::settings::Settings;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use config::{Config, Environment};
use sqlx::postgres::PgPoolOptions;
use std::io::Result;

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
            .app_data(web::Data::new(handlers::AppState { db: pool.clone() }))
            .service(
                web::scope("/events")
                    .route("", web::get().to(handlers::get_events))
                    .route("", web::post().to(handlers::create_event))
                    .route("/{event_id}", web::put().to(handlers::update_event))
                    .route("/{event_id}", web::delete().to(handlers::delete_event))
                    .route("/sync", web::post().to(handlers::sync_events)),
            )
            .route("/healthz", web::get().to(handlers::healthz))
            .route("/activities", web::get().to(handlers::get_activities))
            .route("/destinations", web::get().to(handlers::get_destinations))
    })
    .workers(1)
    .bind(("0.0.0.0", 9336))?
    .run()
    .await
}
