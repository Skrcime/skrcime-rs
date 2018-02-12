
CREATE DOMAIN email_address AS varchar(128) CHECK (value ~ '.+@.+');

CREATE TABLE users (
    id serial PRIMARY KEY,
    name varchar(32) NOT NULL,
    email email_address NOT NULL,
    password text NOT NULL,
    admin boolean DEFAULT false,
    welcome boolean DEFAULT true,
    avatar_url varchar(128),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_email_uq UNIQUE (email)
);
CREATE INDEX email_idx ON users (email);
CREATE INDEX first_name_idx ON users (name);
CREATE INDEX user_creation_idx ON users (id, created_at);
