use axum::{Extension, Json};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    error::AppError,
    models::{CredentialsHashed, CredentialsRaw},
};

pub async fn signup(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(credentials): Json<CredentialsRaw>,
) -> Result<(), AppError> {
    if credentials.login.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    let mut tx = pool.begin().await.map_err(|err| {
        eprintln!("Error: {:?}", err);
        AppError::InternalServerError
    })?;

    let user = sqlx::query_as::<_, CredentialsHashed>(
        "SELECT login, password_hash FROM users WHERE login = $1",
    )
    .bind(&credentials.login)
    .fetch_optional(tx.as_mut())
    .await
    .map_err(|err| {
        eprintln!("Error: {:?}", err);
        AppError::InternalServerError
    })?;

    if user.is_some() {
        return Err(AppError::UserAlreadyExits);
    }

    let result = sqlx::query("INSERT INTO users (login, password_hash) values ($1, $2)")
        .bind(credentials.login)
        .bind(bcrypt::hash(credentials.password, 10).unwrap())
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
    if result.rows_affected() == 0 {
        Err(AppError::InternalServerError)
    } else {
        Ok(())
    }
}
