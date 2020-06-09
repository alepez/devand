FROM rustlang/rust@sha256:2e7212d08e15e685c410e01080c15eb6e733a4fce410dd60f81ab0bcc01b3ead as build

USER root
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y \
      apt-transport-https \
      ca-certificates
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get update
RUN apt-get install -y \
      nodejs
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" > /etc/apt/sources.list.d/yarn.list
RUN apt-get update
RUN apt-get install -y \
      yarn
RUN yarn global add wasm-pack

RUN useradd -m builder

RUN mkdir /home/builder/project
WORKDIR /home/builder/project

ADD --chown=builder ./ ./

USER builder

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
COPY --from=build /home/builder/project/devand-www/static /app/static
COPY --from=build /home/builder/project/devand-www/templates /app/templates
COPY --from=build /home/builder/project/devand-www/target/release/devand-www /bin/devand-www
ENV ROCKET_TEMPLATE_DIR=/app/templates
ENV ROCKET_STATIC_DIR=/app/static
ENTRYPOINT ["/bin/devand-www"]
