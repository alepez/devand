FROM docker.pkg.github.com/alepez/devand/devand-web-builder as builder

WORKDIR /home/builder/project
ADD --chown=builder ./devand-core ./devand-core
ADD --chown=builder ./devand-mailer ./devand-mailer

USER builder
WORKDIR /home/builder/project/devand-mailer
RUN cargo -Z no-index-update build --release --bin=devand-mailer --features=server

# Set up the run environment.
FROM docker.pkg.github.com/alepez/devand/devand-run-env
RUN mkdir /app
COPY --from=builder /home/builder/project/devand-mailer/target/release/devand-mailer /app/devand-mailer
ENTRYPOINT ["/app/devand-mailer"]
