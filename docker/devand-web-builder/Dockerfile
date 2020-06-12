FROM rustlang/rust@sha256:2e7212d08e15e685c410e01080c15eb6e733a4fce410dd60f81ab0bcc01b3ead

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

RUN useradd -m builder

USER builder
RUN yarn global add wasm-pack

