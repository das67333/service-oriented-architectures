use crate::handlers;
use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub struct App {
    db_url: String,
    posts_grpc_port: u16,
    main_port: u16,
}

impl App {
    pub fn new() -> Self {
        let db_user = std::env::var("AUTH_DB_USER").expect("set AUTH_DB_USER env variable");
        let db_password =
            std::env::var("AUTH_DB_PASSWORD").expect("set AUTH_DB_PASSWORD env variable");
        let db_host = std::env::var("AUTH_DB_HOST").expect("set AUTH_DB_HOST env variable");

        let db_url = format!("postgres://{db_user}:{db_password}@{db_host}/{db_user}");

        let posts_grpc_port = std::env::var("POSTS_GRPC_PORT")
            .expect("set POSTS_GRPC_PORT env variable")
            .parse::<u16>()
            .expect("invalid POSTS_GRPC_PORT env variable");

        let main_port = std::env::var("AUTH_PORT")
            .expect("set AUTH_PORT env variable")
            .parse::<u16>()
            .expect("invalid AUTH_PORT env variable");

        Self {
            db_url,
            posts_grpc_port,
            main_port,
        }
    }

    pub async fn run(&self) {
        let users_db_conn_pool = create_db_client(&self.db_url).await;

        handlers::util::try_create_table(&users_db_conn_pool)
            .await
            .expect("Cannot create table");

        let posts_grpc_client = create_grpc_client(self.posts_grpc_port).await;

        let app = Router::new()
            .route("/login", post(handlers::login))
            .route("/signup", post(handlers::signup))
            .route("/profile", put(handlers::update_user))
            .route("/post", post(handlers::create_post))
            .route("/post/:id", put(handlers::update_post))
            .route("/post/:id", delete(handlers::remove_post))
            .route("/post/:id", get(handlers::get_post))
            .route("/posts", get(handlers::get_posts))
            .fallback(handlers::fallback)
            .layer(Extension(Arc::new(users_db_conn_pool)))
            .layer(Extension(Arc::new(Mutex::new(posts_grpc_client))));

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.main_port))
            .await
            .unwrap();

        axum::serve(listener, app).await.unwrap()
    }
}

async fn create_grpc_client(port: u16) -> handlers::GrpcClient {
    const TIMEOUT: Duration = Duration::from_millis(1000);

    let addr = format!("http://posts:{}", port);
    let channel = loop {
        match tonic::transport::Channel::from_shared(addr.clone())
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
                    "Cannot connect to gRPC server: {}. Attempting to reconnect...",
                    err
                );
                sleep(TIMEOUT).await;
            }
        }
    };
    handlers::GrpcClient::new(channel)
}

pub async fn create_db_client(db_url: &str) -> sqlx::PgPool {
    const TIMEOUT: Duration = Duration::from_millis(1000);
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
                    "Cannot connect to database: {}. Attempting to reconnect...",
                    err
                );
                sleep(TIMEOUT).await;
            }
        }
    }
}
