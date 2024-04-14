-- Your SQL goes here
CREATE TABLE vaults (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR NOT NULL,
    encrypted_key TEXT NOT NULL,
    encrypted_data TEXT NOT NULL
);
