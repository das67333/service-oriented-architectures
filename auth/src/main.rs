mod error;
mod handlers;
mod models;

use axum::{extract::Extension, routing::post, Router};
use sqlx::postgres::PgPoolOptions;

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
        .layer(Extension(pool));

    let port = std::env::var("AUTH_PORT")
        .expect("set AUTH_PORT env variable")
        .parse::<u16>()
        .expect("invalid AUTH_PORT env variable");
    let addr = std::net::SocketAddr::from(([0; 4], port));
    dbg!(addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
