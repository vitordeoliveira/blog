FROM rust:slim-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin blog
RUN apt-get install -y ca-certificates

WORKDIR /blog

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./templates/ ./templates/

RUN cargo build --release

COPY ./blogpost/ ./blogpost/
COPY ./assets/ ./assets/
COPY ./sitemap.xml ./sitemap.xml

RUN  cp -rf templates/ ./target/release/. \
  && cp -rf blogpost/ ./target/release/. \
  && cp -rf assets/ ./target/release/. \
  && cp -rf sitemap.xml ./target/release/.

## our final base
FROM debian:stable-slim
ENV SERVER_HOST=0.0.0.0
ENV FIRESTORE_PROJECT_ID=blogpage-416810
ENV SERVER_PORT=8080
ENV RUST_LOG=debug
WORKDIR /app

EXPOSE 8080
COPY --from=build /blog/target/release/ .
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
CMD ["./blog"]
