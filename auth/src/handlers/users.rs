use axum::{http::HeaderMap, Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    error::{internal_server_error, AppError},
    handlers::util::find_user_by_token,
    models::UpdateProfileInput,
    models::{CredentialsHashed, CredentialsRaw},
};

pub async fn signup(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(credentials): Json<CredentialsRaw>,
) -> Result<(), AppError> {
    if credentials.login.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    let mut tx = pool.begin().await.map_err(internal_server_error)?;

    let user = sqlx::query_as::<_, CredentialsHashed>(
        "SELECT login, password_hash FROM users WHERE login = $1",
    )
    .bind(&credentials.login)
    .fetch_optional(tx.as_mut())
    .await
    .map_err(internal_server_error)?;

    if user.is_some() {
        return Err(AppError::UserAlreadyExits);
    }

    sqlx::query("INSERT INTO users (login, password_hash) values ($1, $2)")
        .bind(credentials.login)
        .bind(bcrypt::hash(credentials.password, 10).unwrap())
        .execute(tx.as_mut())
        .await
        .map_err(internal_server_error)?;
    tx.commit().await.map_err(internal_server_error)?;
    Ok(())
}

pub async fn login(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(credentials): Json<CredentialsRaw>,
) -> Result<Json<Value>, AppError> {
    if credentials.login.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    let mut tx = pool.begin().await.map_err(internal_server_error)?;

    let user = sqlx::query_as::<_, CredentialsHashed>(
        "SELECT login, password_hash FROM users where login = $1",
    )
    .bind(&credentials.login)
    .fetch_optional(tx.as_mut())
    .await
    .map_err(internal_server_error)?;

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
                .map_err(internal_server_error)?;
            tx.commit().await.map_err(internal_server_error)?;
            Ok(Json(json!({ "token": token })))
        }
    } else {
        Err(AppError::UserDoesNotExist)
    }
}

pub async fn update_user(
    headers: HeaderMap,
    Extension(pool): Extension<Arc<PgPool>>,
    Json(data): Json<UpdateProfileInput>,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(internal_server_error)?;
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
    .map_err(internal_server_error)?;
    tx.commit().await.map_err(internal_server_error)?;
    Ok(())
}
