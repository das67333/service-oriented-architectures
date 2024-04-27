use axum::{extract::Path, http::HeaderMap, Extension};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use serde_json::json;
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};

use crate::{error::AppError, handlers::util::find_user_by_token};

const KAFKA_MSG_TIMEOUT: Duration = Duration::from_secs(1);

pub async fn view_post(
    headers: HeaderMap,
    Path(id): Path<u64>,
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(kafka_producer): Extension<FutureProducer>,
) -> Result<(), AppError> {
    let login = find_user_by_token(pool.as_ref(), &headers).await?;
    let message = json!({
        "post_id": id,
        "login": login,
    })
    .to_string();

    let record = FutureRecord::<str, str>::to("views").payload(&message);

    kafka_producer
        .send(record, Timeout::After(KAFKA_MSG_TIMEOUT))
        .await
        .map_err(|(err, msg)| {
            eprintln!(
                "Error sending message:\n\terr = {:?},\n\tmsg = {:?}",
                err, msg
            );
            AppError::InternalServerError
        })?;

    Ok(())
}

pub async fn like_post(
    headers: HeaderMap,
    Path(id): Path<u64>,
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(kafka_producer): Extension<FutureProducer>,
) -> Result<(), AppError> {
    let login = find_user_by_token(pool.as_ref(), &headers).await?;
    let message = json!({
        "post_id": id,
        "login": login,
    })
    .to_string();

    let record = FutureRecord::<str, str>::to("likes").payload(&message);

    kafka_producer
        .send(record, Timeout::After(KAFKA_MSG_TIMEOUT))
        .await
        .map_err(|(err, msg)| {
            eprintln!(
                "Error sending message:\n\terr = {:?},\n\tmsg = {:?}",
                err, msg
            );
            AppError::InternalServerError
        })?;

    Ok(())
}
