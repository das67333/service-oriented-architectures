use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    error::AppError,
    models::{CredentialsHashed, CredentialsRaw},
};

pub async fn login(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(credentials): Json<CredentialsRaw>,
) -> Result<Json<Value>, AppError> {
    if credentials.login.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    let mut tx = pool.begin().await.map_err(|err| {
        eprintln!("Error: {:?}", err);
        AppError::InternalServerError
    })?;

    let user = sqlx::query_as::<_, CredentialsHashed>(
        "SELECT login, password_hash FROM users where login = $1",
    )
    .bind(&credentials.login)
    .fetch_optional(tx.as_mut())
    .await
    .map_err(|err| {
        eprintln!("Error: {:?}", err);
        AppError::InternalServerError
    })?;

    if let Some(user) = user {
        if !bcrypt::verify(credentials.password, &user.password_hash).unwrap() {
            Err(AppError::WrongCredential)
        } else {
            let s = format!("{} {}", user.login, chrono::Local::now());
            let token = bcrypt::hash(s, 10).map_err(|_| AppError::TokenCreation)?;
            sqlx::query("UPDATE users SET token = $1 WHERE login = $2")
                .bind(&token)
                .bind(user.login)
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
            Ok(Json(json!({ "token": token })))
        }
    } else {
        Err(AppError::UserDoesNotExist)
    }
}
