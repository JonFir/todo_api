-- Add migration script here
CREATE TABLE users(
   id uuid PRIMARY KEY,
   username TEXT NOT NULL UNIQUE,
   password TEXT NOT NULL,
   salt TEXT NOT NULL,
   email TEXT NOT NULL,
   email_verified BOOL NOT NULL,
   creat TIMESTAMPTZ NOT NULL,
   updat TIMESTAMPTZ NOT NULL,
   is_delete BOOL NOT NULL
);