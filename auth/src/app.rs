use crate::handlers;
use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use rdkafka::{producer::FutureProducer, ClientConfig};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

const INITIAL_TIMEOUT: Duration = Duration::from_millis(1_000);
const TIMEOUT_MULTIPLIER: f64 = 1.2;

pub struct App {
    db_url: String,
    posts_grpc_url: String,
    kafka_url: String,
    main_port: u16,
}

impl App {
    pub fn new() -> Self {
        let [db_user, db_password, db_host, posts_grpc_port, posts_grpc_host, stats_kafka_host, stats_kafka_port, main_port] =
            [
                "AUTH_DB_USER",
                "AUTH_DB_PASSWORD",
                "AUTH_DB_HOST",
                "POSTS_GRPC_PORT",
                "POSTS_GRPC_HOST",
                "STATS_KAFKA_HOST",
                "STATS_KAFKA_PORT",
                "AUTH_PORT",
            ]
            .map(|var| std::env::var(var).expect(&format!("set {} env variable", var)));
        let main_port = main_port
            .parse::<u16>()
            .expect(&format!("invalid AUTH_PORT env variable"));

        let db_url = format!("postgres://{db_user}:{db_password}@{db_host}/{db_user}");
        let posts_grpc_url = format!("http://{posts_grpc_host}:{posts_grpc_port}");
        let kafka_url = format!("{stats_kafka_host}:{stats_kafka_port}");

        Self {
            db_url,
            posts_grpc_url,
            kafka_url,
            main_port,
        }
    }

    pub async fn run(&self) {
        let users_db_conn_pool = create_db_client(&self.db_url).await;

        handlers::util::try_create_table(&users_db_conn_pool)
            .await
            .expect("Cannot create table");

        let posts_grpc_client = create_grpc_client(&self.posts_grpc_url).await;

        let kafka_producer = create_kafka_producer(&self.kafka_url).await;

        let app = Router::new()
            .route("/login", post(handlers::login))
            .route("/signup", post(handlers::signup))
            .route("/profile", put(handlers::update_user))
            .route("/post", post(handlers::create_post))
            .route("/post/:id", put(handlers::update_post))
            .route("/post/:id", delete(handlers::remove_post))
            .route("/post/:id", get(handlers::get_post))
            .route("/posts", get(handlers::get_posts))
            .route("/post/:id/view", post(handlers::view_post))
            .route("/post/:id/like", post(handlers::like_post))
            .fallback(handlers::fallback)
            .layer(Extension(Arc::new(users_db_conn_pool)))
            .layer(Extension(Arc::new(Mutex::new(posts_grpc_client))))
            .layer(Extension(kafka_producer));

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.main_port))
            .await
            .unwrap();

        axum::serve(listener, app).await.unwrap()
    }
}

async fn create_grpc_client(grpc_url: &str) -> handlers::GrpcClient {
    let mut timeout = INITIAL_TIMEOUT;

    let channel = loop {
        match tonic::transport::Channel::from_shared(grpc_url.to_owned())
            .expect("Can't parse address")
            .connect()
            .await
        {
            Ok(pool) => {
                eprintln!("Connected to gRPC server");
                break pool;
            }
            Err(err) => {
                eprintln!(
                    "Cannot connect to gRPC server: \"{}\". Reconnecting in {:.1} seconds...",
                    err,
                    timeout.as_secs_f64()
                );
                sleep(timeout).await;
                timeout = Duration::from_secs_f64(timeout.as_secs_f64() * TIMEOUT_MULTIPLIER);
            }
        }
    };
    handlers::GrpcClient::new(channel)
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
                eprintln!("Connected to database");
                break pool;
            }
            Err(err) => {
                eprintln!(
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
