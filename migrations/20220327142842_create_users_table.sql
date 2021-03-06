-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users(
   id uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
   username TEXT NOT NULL UNIQUE,
   hash TEXT NOT NULL,
   refresh_token TEXT,
   email TEXT NOT NULL UNIQUE,
   email_verified BOOL NOT NULL,
   created TIMESTAMPTZ NOT NULL,
   updated TIMESTAMPTZ NOT NULL,
   is_delete BOOL NOT NULL
);