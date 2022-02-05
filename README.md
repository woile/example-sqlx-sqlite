# example-sqlx-sqlite

Repo used to explore `sqlx`.

Trying to use as different features `sqlx` provides.

## Install sqlx

```sh
cargo install --version=0.5.10 sqlx-cli --features sqlite,tokio-rustls
```

## Create database and run migrations

```sh
./scripts/init_db
```