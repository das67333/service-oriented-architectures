mod app;
mod error;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    app::App::new().run().await;
}
