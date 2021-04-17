CREATE TABLE subscriptions (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscriped_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);