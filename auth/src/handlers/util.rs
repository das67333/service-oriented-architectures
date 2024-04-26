use axum::http::HeaderMap;
use sqlx::{PgPool, Result};

use crate::{error::AppError, models::Login};

pub async fn try_create_table(pool: &PgPool) -> Result<()> {
    // sqlx::query("DROP TABLE IF EXISTS users")
    //     .execute(pool)
    //     .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            login VARCHAR PRIMARY KEY,
            password_hash VARCHAR NOT NULL,
            first_name VARCHAR,
            last_name VARCHAR,
            birth_date DATE,
            email VARCHAR,
            phone VARCHAR,
            token VARCHAR
        )
        ",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_user_by_token(pool: &PgPool, headers: &HeaderMap) -> Result<String, AppError> {
    let token = headers
        .get("authorization")
        .ok_or(AppError::InvalidToken)?
        .to_str()
        .map_err(|_| AppError::InvalidToken)?;
    if token.is_empty() {
        return Err(AppError::InvalidToken);
    }
    let users = sqlx::query_as::<_, Login>("SELECT login FROM users WHERE token = $1 LIMIT 2")
        .bind(token)
        .fetch_all(pool)
        .await
        .map_err(|err| {
            eprintln!("Error: {:?}", err);
            AppError::InternalServerError
        })?;

    if users.is_empty() || users.len() > 1 {
        // >1 means token collision, user should request new token
        return Err(AppError::InvalidToken);
    }
    Ok(users[0].login.clone())
}
