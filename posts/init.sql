CREATE TABLE IF NOT EXISTS posts (
    login VARCHAR,
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP,
    content VARCHAR
)
