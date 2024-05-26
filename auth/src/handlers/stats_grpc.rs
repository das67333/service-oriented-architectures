use axum::{extract::Path, Extension, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::{internal_server_error, AppError};

mod grpc {
    tonic::include_proto!("service_stats");
}

pub use grpc::service_stats_client::ServiceStatsClient;

pub type StatsGrpcClient = ServiceStatsClient<tonic::transport::Channel>;

pub async fn get_post_stats(
    Path(id): Path<u64>,
    Extension(client): Extension<Arc<Mutex<StatsGrpcClient>>>,
) -> Result<Json<Value>, AppError> {
    let arg = grpc::PostId { value: id };
    let response = client
        .lock()
        .await
        .get_post_stats(arg)
        .await
        .map_err(internal_server_error)?;
    Ok(Json(json!({
        "id": id,
        "views": response.get_ref().views,
        "likes": response.get_ref().likes
    })))
}

pub async fn get_top_posts(
    Path(category): Path<String>,
    Extension(client): Extension<Arc<Mutex<StatsGrpcClient>>>,
) -> Result<Json<Value>, AppError> {
    let category =
        grpc::StatCategory::from_str_name(&category.to_uppercase()).ok_or_else(|| {
            tracing::error!("Invalid category: {}", category);
            AppError::InvalidCategory
        })?;
    let arg = grpc::Category {
        value: category as i32,
    };
    let response = client
        .lock()
        .await
        .get_top_posts(arg)
        .await
        .map_err(internal_server_error)?;
    let posts = response.get_ref().posts.clone();
    Ok(Json(json!(posts
        .iter()
        .map(|post| {
            json!({
                "id": post.id,
                "login": post.login,
                "count": post.count
            })
        })
        .collect::<Vec<_>>())))
}

pub async fn get_top_users(
    Extension(client): Extension<Arc<Mutex<StatsGrpcClient>>>,
) -> Result<Json<Value>, AppError> {
    let response = client
        .lock()
        .await
        .get_top_users(())
        .await
        .map_err(internal_server_error)?;
    let users = response.get_ref().users.clone();
    Ok(Json(json!(users
        .iter()
        .map(|user| {
            json!({
                "login": user.login,
                "likes": user.likes,
            })
        })
        .collect::<Vec<_>>())))
}
