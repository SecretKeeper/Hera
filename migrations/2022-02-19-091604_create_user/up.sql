-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    solana_pubkey VARCHAR DEFAULT NULL,
    ethereum_pubkey VARCHAR DEFAULT NULL,
    username VARCHAR DEFAULT NULL,
    email VARCHAR DEFAULT NULL,
    password VARCHAR DEFAULT NULL,
    avatar TEXT DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT null,
    deleted_at TIMESTAMP DEFAULT null
)