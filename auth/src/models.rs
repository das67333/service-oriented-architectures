use serde::Deserialize;
use sqlx::types::chrono::NaiveDate;

#[derive(Deserialize, sqlx::FromRow, Debug, Default)]
pub struct CredentialsRaw {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize, sqlx::FromRow, Debug, Default)]
pub struct CredentialsHashed {
    pub login: String,
    pub password_hash: String,
}

#[derive(serde::Deserialize, sqlx::FromRow, Debug, Default)]
pub struct UpdateInput {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub email: String,
    pub phone: String,
    pub token: String,
}
