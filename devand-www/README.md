# DevAndDev WWW

## Development

A postgres database is needed. It can be created with docker:

```shell
docker run --rm -it -e POSTGRES_PASSWORD=password -p 5432:5432 postgres
```

Database url must be set, by environmental variable or using `dotenv`:

```shell
DATABASE_URL=postgres://postgres:password@localhost/devand >> .env
```

If the database has been just created, it must be set up by `devand-db`.

To launch the server for development (autoreload), just run:

```shell
cargo watch -x "run"
```
