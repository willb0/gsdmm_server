FROM rust:alpine3.14 as build
RUN apk update

WORKDIR /app

RUN apk add musl-dev
ENV CARGO_TARGET_DIR = /target
COPY Cargo.toml Cargo.lock ./
COPY ./src src
RUN mkdir -p .cargo \
  && cargo vendor > .cargo/config

# build dependencies, when my source code changes, this build can be cached, we don't need to compile dependency again.
#--mount=type=cache,target=/usr/local/cargo/registry \
#--mount=type=cache,target=/home/root/app/target \
RUN cargo build
# remove the dummy build.
RUN cargo clean -p gsdmm_server

COPY ./src src

RUN cargo install --locked --path .

EXPOSE 8080
CMD ["gsdmm_server"]