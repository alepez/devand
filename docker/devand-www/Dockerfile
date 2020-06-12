FROM docker.pkg.github.com/alepez/devand/devand-web-builder as builder

USER builder
RUN mkdir /home/builder/project
WORKDIR /home/builder/project

ADD --chown=builder ./ ./

WORKDIR /home/builder/project/devand-ui
RUN bash setup.sh
RUN yarn install
RUN yarn run build

WORKDIR /home/builder/project/devand-www
RUN cargo build --release

# Set up the run environment.
FROM debian:stretch
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
  apt-get -y install \
    libpq5 \
    ca-certificates \
    libssl-dev && \
  rm -rf /var/lib/apt/lists/*
RUN mkdir /app
COPY --from=builder /home/builder/project/devand-www/static /app/static
COPY --from=builder /home/builder/project/devand-www/templates /app/templates
COPY --from=builder /home/builder/project/devand-www/target/release/devand-www /bin/devand-www
ENV ROCKET_TEMPLATE_DIR=/app/templates
ENV ROCKET_STATIC_DIR=/app/static
ENTRYPOINT ["/bin/devand-www"]
