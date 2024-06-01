use crate::handlers;
use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use rdkafka::{producer::FutureProducer, ClientConfig};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

const INITIAL_TIMEOUT: Duration = Duration::from_millis(1_000);
const TIMEOUT_MULTIPLIER: f64 = 1.2;

pub struct App {
    db_url: String,
    posts_grpc_url: String,
    stats_grpc_url: String,
    kafka_url: String,
    main_port: u16,
}

impl App {
    pub fn new() -> Self {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_target(false)
            .compact()
            .init();

        let db_password =
            std::env::var("AUTH_DB_PASSWORD").expect("set AUTH_DB_PASSWORD env variable");

        Self {
            db_url: format!("postgres://postgres:{db_password}@auth_db/postgres"),
            posts_grpc_url: "http://posts:50051".to_owned(),
            stats_grpc_url: "http://stats:50051".to_owned(),
            kafka_url: "stats_kafka:9092".to_owned(),
            main_port: 3000,
        }
    }

    pub async fn run(&self) {
        let users_db_conn_pool = create_db_client(&self.db_url).await;

        handlers::util::init_db(&users_db_conn_pool)
            .await
            .expect("Cannot create table");

        let channel = create_grpc_channel(&self.posts_grpc_url, "posts").await;
        let posts_grpc_client = handlers::PostsGrpcClient::new(channel);

        let channel = create_grpc_channel(&self.stats_grpc_url, "stats").await;
        let stats_grpc_client = handlers::StatsGrpcClient::new(channel);

        let kafka_producer = create_kafka_producer(&self.kafka_url).await;

        let app = Router::new()
            .route("/signup", post(handlers::signup))
            .route("/login", post(handlers::login))
            .route("/profile", put(handlers::update_user))
            .route("/post", post(handlers::create_post))
            .route("/post/:id", put(handlers::update_post))
            .route("/post/:id", delete(handlers::remove_post))
            .route("/post/:id", get(handlers::get_post))
            .route("/posts", get(handlers::get_posts))
            .route("/post/:id/view", post(handlers::view_post))
            .route("/post/:id/like", post(handlers::like_post))
            .route("/stats/post/:id", get(handlers::get_post_stats))
            .route("/stats/top_posts/:category", get(handlers::get_top_posts))
            .route("/stats/top_users", get(handlers::get_top_users))
            .fallback(handlers::fallback)
            .layer(Extension(Arc::new(users_db_conn_pool)))
            .layer(Extension(Arc::new(Mutex::new(posts_grpc_client))))
            .layer(Extension(Arc::new(Mutex::new(stats_grpc_client))))
            .layer(Extension(kafka_producer))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            );

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.main_port))
            .await
            .unwrap();

        axum::serve(listener, app).await.unwrap()
    }
}

async fn create_grpc_channel(grpc_url: &str, name: &str) -> tonic::transport::Channel {
    let mut timeout = INITIAL_TIMEOUT;
    loop {
        match tonic::transport::Channel::from_shared(grpc_url.to_owned())
            .expect("Can't parse address")
            .connect()
            .await
        {
            Ok(pool) => {
                tracing::info!("Connected to {} gRPC server", name);
                break pool;
            }
            Err(err) => {
                tracing::warn!(
                    "Cannot connect to {} gRPC server: \"{}\". Reconnecting in {:.1} seconds...",
                    name,
                    err,
                    timeout.as_secs_f64()
                );
                sleep(timeout).await;
                timeout = Duration::from_secs_f64(timeout.as_secs_f64() * TIMEOUT_MULTIPLIER);
            }
        }
    }
}

async fn create_db_client(db_url: &str) -> sqlx::PgPool {
    let mut timeout = INITIAL_TIMEOUT;

    const MAX_CONNECTIONS: u32 = 5;
    loop {
        match PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(db_url)
            .await
        {
            Ok(pool) => {
                tracing::info!("Connected to database");
                break pool;
            }
            Err(err) => {
                tracing::warn!(
                    "Cannot connect to database: \"{}\". Reconnecting in {:.1} seconds...",
                    err,
                    timeout.as_secs_f64()
                );
                sleep(timeout).await;
                timeout = Duration::from_secs_f64(timeout.as_secs_f64() * TIMEOUT_MULTIPLIER);
            }
        }
    }
}

async fn create_kafka_producer(kafka_url: &str) -> FutureProducer {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_url)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka producer");
    producer
}
