## About

TODO

## Usage

### Build

When building for the first time, ensure to install dependencies first.

```shell
bash setup.sh
yarn install
```

#### Build for devand-web

```shell
yarn run build
```

Generated files are copied in `../devand-web/static/ui` directory

### Serve locally

This configuration does not need a server or a database, mock services
are automatically instantiated.

```shell
yarn run start:dev
```

Server will be available at `http://localhost:8001` with autoreload enabled.
