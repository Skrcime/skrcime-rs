-- You will need this extension installed
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE DOMAIN email_address AS varchar(128) CHECK (value ~ '.+@.+');

CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    first_name varchar(32) NOT NULL,
    last_name varchar(32) NOT NULL,
    email email_address NOT NULL,
    password text NOT NULL,
    admin boolean DEFAULT false,
    welcome boolean DEFAULT true,
    avatar_url varchar(128),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_email_uq UNIQUE (email)
);
CREATE INDEX email_idx ON users (email);
CREATE INDEX first_name_idx ON users (first_name);
CREATE INDEX last_name_idx ON users (last_name);
CREATE INDEX user_creation_idx ON users (id, created_at);
