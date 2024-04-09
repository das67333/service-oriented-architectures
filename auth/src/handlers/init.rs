use sqlx::{PgPool, Result};

pub async fn try_create_table(pool: &PgPool) -> Result<()> {
    // sqlx::query("DROP TABLE IF EXISTS users")
    //     .execute(pool)
    //     .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            login VARCHAR NOT NULL UNIQUE,
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
