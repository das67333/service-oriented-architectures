use sqlx::types::chrono::NaiveDate;

#[derive(serde::Deserialize, sqlx::FromRow)]
pub struct CredentialsRaw {
    pub login: String,
    pub password: String,
}

#[derive(serde::Deserialize, sqlx::FromRow)]
pub struct CredentialsHashed {
    pub login: String,
    pub password_hash: String,
}

#[derive(serde::Deserialize, sqlx::FromRow)]
pub struct UpdateInput {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub email: String,
    pub phone: String,
}

#[derive(serde::Deserialize, sqlx::FromRow)]
pub struct Login {
    pub login: String,
}
