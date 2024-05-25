use axum::{extract::Path, http::HeaderMap, Extension};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use serde_json::json;
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};

use crate::{error::AppError, handlers::util::find_user_by_token};

const KAFKA_MSG_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn view_post(
    headers: HeaderMap,
    Path(id): Path<u64>,
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(kafka_producer): Extension<FutureProducer>,
) -> Result<(), AppError> {
    let login = find_user_by_token(pool.as_ref(), &headers).await?;

    tokio::spawn(async move {
        let message = json!({
            "post_id": id,
            "login": login,
        })
        .to_string();
        let record = FutureRecord::<str, str>::to("views").payload(&message);
        match kafka_producer
            .send(record, Timeout::After(KAFKA_MSG_TIMEOUT))
            .await
        {
            Ok((partition, offset)) => {
                tracing::info!(
                    "\"View\" delivery confirmed:\n\tpartition = {}, offset = {}\n\tmessage = {}",
                    partition,
                    offset,
                    message
                );
            }
            Err((err, msg)) => {
                tracing::error!(
                    "\"View\" delivery failed:\n\terr = {:?},\n\tmessage = {:?}",
                    err,
                    msg
                );
            }
        }
    });

    Ok(())
}

pub async fn like_post(
    headers: HeaderMap,
    Path(id): Path<u64>,
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(kafka_producer): Extension<FutureProducer>,
) -> Result<(), AppError> {
    let login = find_user_by_token(pool.as_ref(), &headers).await?;

    tokio::spawn(async move {
        let message = json!({
            "post_id": id,
            "login": login,
        })
        .to_string();
        let record = FutureRecord::<str, str>::to("likes").payload(&message);
        match kafka_producer
            .send(record, Timeout::After(KAFKA_MSG_TIMEOUT))
            .await
        {
            Ok((partition, offset)) => {
                tracing::info!(
                    "\"Like\" delivery confirmed:\n\tpartition = {}, offset = {}\n\tmessage = {}",
                    partition,
                    offset,
                    message
                );
            }
            Err((err, msg)) => {
                tracing::error!(
                    "\"Like\" delivery failed:\n\terr = {:?},\n\tmessage = {:?}",
                    err,
                    msg
                );
            }
        }
    });

    Ok(())
}
