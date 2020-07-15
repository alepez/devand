# devand-ui

## Build

When building for the first time, ensure to install dependencies first.

```shell
bash setup.sh
yarn install
```

### Build for devand-web

```shell
yarn run build
```
If the builder find the rust stable release report the ```error[E0554]: #![feature] may not be used on the stable release channel```.
So it's necessary to install and use, by default, the rust nighlty version.

Generated files are copied in `../devand-web/static/ui` directory

## Serve locally

This configuration does not need a server or a database, mock services
are automatically instantiated.

```shell
yarn run start:dev
```

Server will be available at `http://localhost:8001` with autoreload enabled.
