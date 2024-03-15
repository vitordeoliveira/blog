FROM rust:slim-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin blog
WORKDIR /blog

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./templates/ ./templates/

RUN cargo build --release

COPY ./blogpost/ ./blogpost/
COPY ./assets/ ./assets/

RUN  cp -rf templates/ ./target/release/. \
  && cp -rf blogpost/ ./target/release/. \
  && cp -rf assets/ ./target/release/.

## our final base
FROM debian:stable-slim
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
ENV RUST_LOG=debug
WORKDIR /app

EXPOSE 8080
COPY --from=build /blog/target/release/ .
CMD ["./blog"]
