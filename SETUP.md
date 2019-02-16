# Env

Create `.env` file in the root of this project

```
ROCKET_SECRET_KEY=<RANDOM_HASH>
DATABASE_URL=postgres://skrcime:@localhost/skrcime
```

# Rust

Install `rustup` and run

```
rustup install nightly
rustup default nightly
cargo build
```

# PostgreSQL

Install postgresql server

```
sudo -iu postgres
initdb -D /var/lib/postgres/data
systemctrl start postgresql
createuser --interactive
createdb skrcime
```

# Diesel

Install diesel CLI tool

```
cargo install diesel_cli
diesel migration run
```

# Nginx

Install nginx and start it `systemctrl start nginx`

Update `/etc/nginx/nginx.config` file by adding

```
user <USER_NAME>;

...

server {
  listen 80;
  server_name skrci.me moj.skrci.me;

  location /static/ {
    alias /home/nilg/code/skrcime-rs/static/;
  }
  location / {
    proxy_pass http://127.0.0.1:8000;
    proxy_set_header Host $host;
  }
}
```

# Hosts

Update `/etc/hosts` file

```
127.0.0.1 skrci.me
127.0.0.1 moj.skrci.me
```