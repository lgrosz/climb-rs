# climb-db

A database for climbs.

## Development

This library expects access to PostgreSQL server. Set one up like so...

```sh
initdb -D test-db
pg_ctl -D test-db -l logfile start
```

... initialize the database with [Diesel](https://diesel.rs)...

```sh
echo DATABASE_URL=postgres://localhost:5432/climb-db > .env
diesel setup
```
