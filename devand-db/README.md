# devand-db

## Development

### Setup `diesel_cli`

```shell
cargo install diesel_cli --no-default-features --features postgres
```

### Setup postgres database

```shell
docker run --rm -it -e POSTGRES_PASSWORD=password -p 5432:5432 postgres
echo DATABASE_URL=postgres://postgres:password@localhost/devand > .env
diesel setup
```
