FROM rust:alpine as build-image

RUN apk add --no-cache musl-dev

# create a new empty shell project
RUN USER=root cargo new --bin backend
WORKDIR /backend

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm -f ./target/x86_64-unknown-linux-musl/release/deps/backend* && \
    rm -f ./target/release/deps/backend*
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest 
RUN apk add --no-cache musl-dev
WORKDIR /backend

COPY --from=build-image /backend/target/x86_64-unknown-linux-musl/release/backend /backend/app

ENTRYPOINT /backend/app
