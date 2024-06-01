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
