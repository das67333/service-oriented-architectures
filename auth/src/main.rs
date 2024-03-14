mod error;
mod handlers;
mod models;

use axum::{extract::Extension, routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let durl = std::env::var("AUTH_DB_URL").expect("set AUTH_DB_URL env variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to connect to database");

    handlers::try_create_table(&pool)
        .await
        .expect("Cannot create table");

    let app = Router::new()
        .route("/login", post(handlers::login))
        .route("/signup", post(handlers::signup))
        .route("/update", post(handlers::update))
        .layer(Extension(Arc::new(pool)));

    let port = std::env::var("AUTH_PORT")
        .expect("set AUTH_PORT env variable")
        .parse::<u16>()
        .expect("invalid AUTH_PORT env variable");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap()
}
