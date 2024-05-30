use axum::http::HeaderMap;
use sqlx::{PgPool, Result};

use crate::{
    error::{internal_server_error, AppError},
    models::Login,
};

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

fn get_login_if_single(users: &[Login]) -> Result<String, AppError> {
    if users.is_empty() || users.len() > 1 {
        return Err(AppError::InvalidToken);
    }
    Ok(users[0].login.clone())
}

pub async fn find_user_by_token<'a>(
    e: impl sqlx::PgExecutor<'a>,
    headers: &HeaderMap,
) -> Result<String, AppError> {
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
        .fetch_all(e)
        .await
        .map_err(internal_server_error)?;

    // >1 means token collision, user should request new token
    get_login_if_single(&users)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_login_if_single() {
        let mut users = vec![];
        assert!(get_login_if_single(&users).is_err());

        users.push(Login {
            login: "test_user1".to_owned(),
        });
        assert_eq!(get_login_if_single(&users).unwrap(), "test_user1");

        users.push(Login {
            login: "test_user2".to_owned(),
        });
        assert!(get_login_if_single(&users).is_err());
    }
}
