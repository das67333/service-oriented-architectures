use axum::{extract::Path, http::HeaderMap, Extension, Json};
use axum_extra::extract::Query;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    error::{internal_server_error, AppError},
    handlers::util::find_user_by_token,
};

mod grpc {
    tonic::include_proto!("service_posts");
}

use grpc::service_posts_client::ServicePostsClient;

pub type PostsGrpcClient = ServicePostsClient<tonic::transport::Channel>;

pub async fn create_post(
    headers: HeaderMap,
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(client): Extension<Arc<Mutex<PostsGrpcClient>>>,
    Json(data): Json<restapi::RequestCreate>,
) -> Result<Json<Value>, AppError> {
    let arg = grpc::RequestCreate {
        login: find_user_by_token(pool.as_ref(), &headers).await?,
        content: data.content,
    };
    let response = client
        .lock()
        .await
        .create_post(arg)
        .await
        .map_err(internal_server_error)?;
    let post_id = response.get_ref().value;
    Ok(Json(json!({ "post_id": post_id })))
}

pub async fn update_post(
    headers: HeaderMap,
    Path(id): Path<u64>,
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(client): Extension<Arc<Mutex<PostsGrpcClient>>>,
    Json(data): Json<restapi::RequestUpdate>,
) -> Result<(), AppError> {
    let arg = grpc::RequestUpdate {
        login: find_user_by_token(pool.as_ref(), &headers).await?,
        id,
        content: data.content,
    };
    let response = client
        .lock()
        .await
        .update_post(arg)
        .await
        .map_err(internal_server_error)?;
    match response.get_ref().code() {
        grpc::Status::LoginMismatch => Err(AppError::AccessDenied),
        grpc::Status::PostNotFound => Err(AppError::PostNotFound),
        grpc::Status::Ok => Ok(()),
        _ => Err(AppError::InternalServerError),
    }
}

pub async fn remove_post(
    headers: HeaderMap,
    Path(id): Path<u64>,
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(client): Extension<Arc<Mutex<PostsGrpcClient>>>,
) -> Result<(), AppError> {
    let arg = grpc::RequestRemove {
        login: find_user_by_token(pool.as_ref(), &headers).await?,
        id,
    };
    let response = client
        .lock()
        .await
        .remove_post(arg)
        .await
        .map_err(internal_server_error)?;
    match response.get_ref().code() {
        grpc::Status::LoginMismatch => Err(AppError::AccessDenied),
        grpc::Status::PostNotFound => Err(AppError::PostNotFound),
        grpc::Status::Ok => Ok(()),
        _ => Err(AppError::InternalServerError),
    }
}

pub async fn get_post(
    Path(id): Path<u64>,
    Extension(client): Extension<Arc<Mutex<PostsGrpcClient>>>,
) -> Result<Json<Value>, AppError> {
    let arg = grpc::RequestGetOne { id };
    let response = client
        .lock()
        .await
        .get_post(arg)
        .await
        .map_err(internal_server_error)?;
    match response.get_ref().code() {
        grpc::Status::PostNotFound => Err(AppError::PostNotFound),
        grpc::Status::Ok => {
            let post = response.get_ref().post.as_ref().expect("broken invariant");
            Ok(Json(json!({
                "login": post.login,
                "created_at": post.created_at.as_ref().expect("broken invariant").to_string(),
                "content": post.content,
            })))
        }
        _ => Err(AppError::InternalServerError),
    }
}

pub async fn get_posts(
    Extension(client): Extension<Arc<Mutex<PostsGrpcClient>>>,
    Query(params): Query<restapi::PostsRange>,
) -> Result<Json<Value>, AppError> {
    let arg = grpc::RequestGetMany {
        login: params.login,
        start_id: params.start_id,
        count: params.count,
    };
    let response = client
        .lock()
        .await
        .get_posts(arg)
        .await
        .map_err(internal_server_error)?;
    match response.get_ref().code() {
        grpc::Status::PostNotFound => Err(AppError::PostNotFound),
        grpc::Status::Ok => {
            let posts = response.get_ref().posts.clone();
            Ok(Json(json!(posts
                .iter()
                .map(|post| {
                    json!({
                        "id": post.id,
                        "created_at": post.created_at.as_ref().expect("broken invariant").to_string(),
                        "content": post.content,
                    })
                })
                .collect::<Vec<_>>())))
        }
        _ => Err(AppError::InternalServerError),
    }
}

mod restapi {
    #[derive(serde::Deserialize)]
    pub struct RequestCreate {
        pub content: String,
    }

    #[derive(serde::Deserialize)]
    pub struct RequestUpdate {
        pub content: String,
    }

    #[derive(serde::Deserialize)]
    pub struct PostsRange {
        pub login: String,
        pub start_id: u64,
        pub count: u64,
    }
}
