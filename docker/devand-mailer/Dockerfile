FROM docker.pkg.github.com/alepez/devand/devand-web-builder as builder

WORKDIR /home/builder/project
ADD --chown=builder ./devand-core ./devand-core
ADD --chown=builder ./devand-crypto ./devand-crypto
ADD --chown=builder ./devand-db ./devand-db
ADD --chown=builder ./devand-mailer ./devand-mailer
ADD --chown=builder ./devand-text ./devand-text

USER builder
WORKDIR /home/builder/project/devand-mailer
RUN cargo -Z no-index-update build --release --bin=devand-mailer --features=server
RUN cargo -Z no-index-update build --release --bin=devand-verify-address-reminder --features=client

# Set up the run environment.
FROM docker.pkg.github.com/alepez/devand/devand-run-env
COPY --from=builder /home/builder/project/devand-mailer/target/release/devand-mailer /app/devand-mailer
COPY --from=builder /home/builder/project/devand-mailer/target/release/devand-verify-address-reminder /app/devand-verify-address-reminder
ENTRYPOINT ["/app/devand-mailer"]
