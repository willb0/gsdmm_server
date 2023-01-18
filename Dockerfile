FROM rust:alpine3.14 as build

WORKDIR /app

RUN apk add musl-dev
ENV CARGO_TARGET_DIR = /target
COPY Cargo.toml Cargo.lock ./
COPY ./src src

# build dependencies, when my source code changes, this build can be cached, we don't need to compile dependency again.
RUN cargo build --release
# remove the dummy build.
RUN cargo clean -p gsdmm_server

COPY ./src src

RUN cargo install --path . 

# second stage.
FROM alpine:3.14
COPY --from=build /usr/local/cargo/bin/* /usr/local/bin/
EXPOSE 8080
CMD ["gsdmm_server"]





