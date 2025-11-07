CREATE TABLE devices (
    id SERIAL PRIMARY KEY,
    address TEXT UNIQUE NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);
