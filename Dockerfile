FROM --platform=$BUILDPLATFORM rust:alpine3.14 as build

WORKDIR /app

RUN apk add musl-dev
ENV CARGO_TARGET_DIR = /target
COPY Cargo.toml Cargo.lock ./
COPY ./src src

# build dependencies, when my source code changes, this build can be cached, we don't need to compile dependency again.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    cargo build
# remove the dummy build.
RUN cargo clean -p gsdmm_server

COPY ./src src

RUN cargo install --path . --target=aarch64-unknown-linux-musl

# second stage.
FROM --platform=$BUILDPLATFORM alpine:3.14
COPY --from=build /usr/local/cargo/bin/* /usr/local/bin
EXPOSE 8080
CMD ["gsdmm_server"]





