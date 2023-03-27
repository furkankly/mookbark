FROM --platform=linux/amd64 rust:1.76-bullseye as dependencies 

WORKDIR /mookbark
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./termtree ./termtree
COPY ./tui-rs-tree-widget ./tui-rs-tree-widget

# Create a dummy src directory to prevent cargo new from creating one
RUN mkdir src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build --release

# Now build the actual application
FROM dependencies as builder

# Build web app with own code
RUN rm src/*.rs
COPY . .
RUN rm ./target/release/deps/mookbark*
RUN cargo build --release

ENV NVM_DIR /root/.nvm
ENV NODE_VERSION 18.18.0

RUN rm -rf client/apps/web/dist && \  
  apt-get update && \
  apt-get install -y curl && \
  apt-get -y autoclean
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"
RUN node --version && \
  npm --version  && \
  npm install -g yarn && \
  cd client/apps/web && \
  yarn && \
  yarn build

FROM --platform=linux/amd64 debian:bullseye-slim

ARG APP_DIR=/usr/src/app
ENV APP_USER="appuser" SERVER_ENV="production"
WORKDIR ${APP_DIR}

COPY --from=builder /mookbark/target/release/mookbark .
COPY --from=builder /mookbark/client/apps/web/dist ./client/apps/web/dist
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates

RUN groupadd $APP_USER && \
  useradd -g $APP_USER $APP_USER && \
  chown -R $APP_USER:$APP_USER ${APP_DIR}

USER $APP_USER
EXPOSE 8080
ENTRYPOINT ["./mookbark", "server"]
