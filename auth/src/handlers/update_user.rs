use axum::{http::HeaderMap, Extension, Json};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{error::AppError, handlers::util::find_user_by_token, models::UpdateInput};

pub async fn update_user(
    headers: HeaderMap,
    Extension(pool): Extension<Arc<PgPool>>,
    Json(data): Json<UpdateInput>,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(|err| {
        eprintln!("Error: {:?}", err);
        AppError::InternalServerError
    })?;

    let login = find_user_by_token(tx.as_mut(), &headers).await?;

    sqlx::query(
        "
        UPDATE users
        SET first_name = $1, last_name = $2, birth_date = $3, email = $4, phone = $5
        WHERE login = $6
        ",
    )
    .bind(data.first_name)
    .bind(data.last_name)
    .bind(data.birth_date)
    .bind(data.email)
    .bind(data.phone)
    .bind(login)
    .execute(tx.as_mut())
    .await
    .map_err(|err| {
        eprintln!("Error: {:?}", err);
        AppError::InternalServerError
    })?;
    tx.commit().await.map_err(|err| {
        eprintln!("Error: {:?}", err);
        AppError::InternalServerError
    })?;
    Ok(())
}
