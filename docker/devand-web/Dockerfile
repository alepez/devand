FROM docker.pkg.github.com/alepez/devand/devand-web-builder as builder

WORKDIR /home/builder/project
ADD --chown=builder ./devand-core ./devand-core
ADD --chown=builder ./devand-db ./devand-db
ADD --chown=builder ./devand-ui ./devand-ui
ADD --chown=builder ./devand-web ./devand-web
ADD --chown=builder ./devand-mailer ./devand-mailer

USER builder
WORKDIR /home/builder/project/devand-ui
RUN bash setup.sh && \
    yarn install && \
    yarn run build

WORKDIR /home/builder/project/devand-web
RUN cargo -Z no-index-update build --release

# Set up the run environment.
FROM docker.pkg.github.com/alepez/devand/devand-run-env
RUN mkdir /app
COPY --from=builder /home/builder/project/devand-web/static /app/static
COPY --from=builder /home/builder/project/devand-web/templates /app/templates
COPY --from=builder /home/builder/project/devand-web/target/release/devand-web /app/devand-web
ENV ROCKET_TEMPLATE_DIR=/app/templates
ENV ROCKET_STATIC_DIR=/app/static
ENTRYPOINT ["/app/devand-web"]
