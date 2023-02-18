CREATE TABLE IF NOT EXISTS users
(
    id          BINARY(16) PRIMARY KEY NOT NULL UNIQUE DEFAULT (UUID()),
    first_name  VARCHAR(50) NOT NULL,
    last_name   VARCHAR(50) NOT NULL,
    email       VARCHAR(256) NOT NULL UNIQUE,
    password    VARCHAR(64) NOT NULL,
    salt        VARCHAR(64) NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL
);
