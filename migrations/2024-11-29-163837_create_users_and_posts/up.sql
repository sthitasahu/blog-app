-- Your SQL goes here
-- up.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR NOT NULL UNIQUE,
    name VARCHAR,
    password VARCHAR NOT NULL
);

CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN DEFAULT false,
    published_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    author_id UUID REFERENCES users(id) ON DELETE CASCADE
);
