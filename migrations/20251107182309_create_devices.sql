CREATE TABLE devices (
    id BIGSERIAL PRIMARY KEY,
    address TEXT UNIQUE NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);
