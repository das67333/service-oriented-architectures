use axum::{Extension, Json};
use sqlx::PgPool;

use crate::{
    error::AppError,
    models::{CredentialsHashed, CredentialsRaw},
};

pub async fn signup(
    Json(credentials): Json<CredentialsRaw>,
    Extension(pool): Extension<PgPool>,
) -> Result<(), AppError> {
    if credentials.login.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    let user = sqlx::query_as::<_, CredentialsHashed>(
        "SELECT login, password_hash FROM users WHERE login = $1",
    )
    .bind(&credentials.login)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if user.is_some() {
        return Err(AppError::UserAlreadyExits);
    }

    let result = sqlx::query("INSERT INTO users (login, password_hash) values ($1, $2)")
        .bind(credentials.login)
        .bind(bcrypt::hash(credentials.password, 10).unwrap())
        .execute(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;
    if result.rows_affected() == 0 {
        Err(AppError::InternalServerError)
    } else {
        Ok(())
    }
}
