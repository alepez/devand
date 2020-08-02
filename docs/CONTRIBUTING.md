# Contributing

Contributions are absolutely, positively welcome and encouraged! Contributions
come in many forms. You could:

  1. Submit a feature request or bug report as an [issue].
  2. Ask for improved documentation as an [issue].
  3. Comment on [issues that require feedback].
  4. Contribute code via [pull requests].
  5. Propose a pair-programming session to [alepez](https://devand.dev/chat/alepez)

[issue]: https://github.com/alepez/devand/issues
[issues that require feedback]: https://github.com/alepez/devand/issues?q=is%3Aissue+is%3Aopen+label%3A%22feedback+wanted%22
[pull requests]: https://github.com/alepez/devand/pulls

## Setting up your local development environment

Project is currently built with `rustc 1.46.0-nightly`. This is needed by
`rocket`. It needs to be built both for `wasm32-unknown-unknown` and
`x86_64-unknown-linux-gnu` targets.

To build `devand-ui` (Yew frontend) add `wasm32-unknown-unknown` environment:

```shell
rustup target add wasm32-unknown-unknown
```

`webpack` and `yarn` are also needed. Refer to `devand-ui` for additional
documentation about dependencies.

Project can be built inside a Docker container too, see
[the Dockerfile](/docker/devand-web-builder/Dockerfile)
for a complete build environment.

`devand-ui` can be tested without any backend, it just uses *mock* services to
retrieve fake data.

Running of `devand-web` requires two steps: first you need to build `devand-ui`
(js, wasm, css files are generated and copied to `devand-web`), then you can
`cargo run` inside `devand-web` directory. `devand-web` needs a running
postgresql server. A secret must also be provided, it is used to sign and verify
tokens like password recovery url. Secret and PostgreSQL credentials must be
written in a `.env` like this:

```txt
DATABASE_URL=postgres://postgres:password@localhost/devand
DEVAND_SECRET=shii3AhQuahho1shaeW7jooWeeMaYahS
```

Change password and secret with anything valid.

PostgreSQL server can be launched with Docker:

```shell
docker run --rm -ti \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=devand \
  -p 5432:5432 \
  --name devand_db postgres
```

To build `devand-ui` and start `devand-web`:

```shell
( cd devand-ui && yarn run build )
( cd devand-web && cargo watch -x check -x test -x run )
```
Be sure that cargo watch is installed otherwise the building process end with an error.
You can install it executing ```cargo install cargo-watch```

To create and test `devand-db` database migrations, a `.env` file must be
created inside `devand-db` directory, with this content:

```shell
DATABASE_URL=postgres://postgres:password@localhost/devand
```

To test `devand-mailer` a `.env` file must be created inside `devand-mailer`
directory, with this content:

```shell
DEVAND_SECRET=shii3AhQuahho1shaeW7jooWeeMaYahS
DEVAND_MAILER_SMTP_SERVER=mail.example.com
DEVAND_MAILER_SMTP_USERNAME=noreply@example.com
DEVAND_MAILER_SMTP_PASSWORD=lasdfkjghladfikhjgsol
DEVAND_MAILER_RPC_HTTP_ADDR=0.0.0.0:3030
DEVAND_MAILER_SERVER_URL=http://127.0.0.1:3030
DATABASE_URL=postgres://postgres:password@localhost/devand
```

Change smtp address and credentials as needed. `DEVAND_SECRET` must be the same
used by `devand-web`.

## Setting up local environment with docker
In folder `development` there is an example of `.env` files and a `docker-compose.yml` to set up 4 docker container used to try it on localhost:8000.

- `cd development`
- `docker-compose up`
- connect to `localhost:8000` on you browser

Please make attention to:
- file in `db.env` `PGADMIN_DEFAULT_EMAIL` use a valid email
- to generate secrets you can use `openssl rand -base64 32`