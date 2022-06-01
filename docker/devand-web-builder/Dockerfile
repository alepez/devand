FROM rustlang/rust@sha256:82c6bff2e60150c13e737266eb4e917f503ac5eee39d3264e74a3b141bd32589

ENV DEBIAN_FRONTEND=noninteractive

# First apt-transport-https and ca-certificates must be installed
# Then we can add nodejs and yarn apt sources
# Then apt-get update must be run again
# Then we can install nodejs and yarn
RUN apt-get update && \
    apt-get install -y \
      apt-transport-https \
      ca-certificates && \
    curl -sSL https://deb.nodesource.com/setup_16.x | bash - && \
    curl -sSL https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && \
    echo "deb https://dl.yarnpkg.com/debian/ stable main" > /etc/apt/sources.list.d/yarn.list && \
    apt-get update && \
    apt-get install -y \
      nodejs yarn && \
    useradd -m builder

# wasm-pack must be installed by user `builder`
# use `builder` must be used in derivative images to build cargo projects
USER builder
RUN yarn global add wasm-pack

# Update cargo index, so it is ready for images extending this one
RUN cargo search --limit 0

USER builder
RUN mkdir /home/builder/project
