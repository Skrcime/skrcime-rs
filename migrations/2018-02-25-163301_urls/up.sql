
CREATE TABLE urls (
    id serial PRIMARY KEY,
    target text NOT NULL,
    hash varchar(32) NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at timestamp,
    CONSTRAINT hash_uq UNIQUE (hash)
);
CREATE INDEX target_idx ON urls (target);
CREATE INDEX hash_idx ON urls (hash);


CREATE TABLE user_urls (
    user_id serial REFERENCES users (id),
    url_id serial REFERENCES urls (id),
    CONSTRAINT user_urls_uq UNIQUE (user_id, url_id)
);
CREATE INDEX user_urls_idx ON user_urls (user_id);
