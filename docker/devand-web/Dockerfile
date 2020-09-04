FROM docker.pkg.github.com/alepez/devand/devand-web-builder as builder

WORKDIR /home/builder/project
ADD --chown=builder ./Cargo.toml ./Cargo.toml
ADD --chown=builder ./Cargo.lock ./Cargo.lock
ADD --chown=builder ./devand-core ./devand-core
ADD --chown=builder ./devand-crypto ./devand-crypto
ADD --chown=builder ./devand-db ./devand-db
ADD --chown=builder ./devand-mailer ./devand-mailer
ADD --chown=builder ./devand-ui ./devand-ui
ADD --chown=builder ./devand-text ./devand-text
ADD --chown=builder ./devand-web ./devand-web

USER builder
WORKDIR /home/builder/project/devand-ui
RUN bash setup.sh && \
    yarn install && \
    yarn run build

WORKDIR /home/builder/project/devand-web
ENV DEVAND_BASE_URL=https://devand.dev
RUN cargo -Z no-index-update build --release
RUN ./tools/replace-hash-placeholder

# Set up the run environment.
FROM docker.pkg.github.com/alepez/devand/devand-run-env
COPY --from=builder /home/builder/project/devand-web/static /app/static
COPY --from=builder /home/builder/project/devand-web/templates /app/templates
COPY --from=builder /home/builder/project/target/release/devand-web /app/devand-web
ENV ROCKET_TEMPLATE_DIR=/app/templates
ENV ROCKET_STATIC_DIR=/app/static
ENTRYPOINT ["/app/devand-web"]
