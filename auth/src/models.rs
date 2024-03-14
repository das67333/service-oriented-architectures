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
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub token: String,
}
