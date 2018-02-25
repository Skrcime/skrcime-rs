
CREATE TABLE urls (
    id serial PRIMARY KEY,
    hash varchar(32) NOT NULL,
    url text NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at timestamp,
);
CREATE INDEX hash_idx ON urls (hash);
CREATE INDEX url_idx ON urls (url);


CREATE TABLE user_urls (
    user_id serial NOT NULL,
    url_id serial NOT NULL,
    CONSTRAINT user_urls_uq UNIQUE (user_id, url_id)
);
CREATE INDEX user_urls_idx ON user_urls (user_id);
