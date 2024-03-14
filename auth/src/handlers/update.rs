use axum::{Extension, Json};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{error::AppError, models::UpdateInput};

pub async fn update(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(data): Json<UpdateInput>,
) -> Result<(), AppError> {
    let token = data.token;
    if token.is_empty() {
        return Err(AppError::InvalidToken);
    }

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE token = $1")
        .bind(&token)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;

    if count == 0 || count > 1 {
        // count > 1 means token collision, user should request new token
        return Err(AppError::InvalidToken);
    }

    sqlx::query("UPDATE users SET first_name = $1, last_name = $2, birth_date = $3, email = $4, phone = $5 WHERE token = $6")
        .bind(data.first_name)
        .bind(data.last_name)
        .bind(data.birth_date)
        .bind(data.email)
        .bind(data.phone)
        .bind(token)
        .execute(pool.as_ref())
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;
    Ok(())
}
