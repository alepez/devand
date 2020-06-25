# Generate a common environment used by all devand services
FROM debian:stretch
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
  apt-get -y install \
    libpq5 \
    ca-certificates \
    libssl-dev && \
  rm -rf /var/lib/apt/lists/* && \
  mkdir /app
