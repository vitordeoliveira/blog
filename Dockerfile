FROM rust:1.77.1 as build


ARG SQLITE_DB=/blog/data/blog.sqlite
ARG SERVER_HOST=0.0.0.0
ARG SERVER_PORT=8080
ARG RUST_LOG=debug

RUN apt-get update && apt-get install -y musl-tools
WORKDIR /blog

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./templates/ ./templates/
COPY ./migrations/ ./migrations/
COPY ./blogpost/ ./blogpost/
COPY ./assets/ ./assets/
COPY ./sitemap.xml ./sitemap.xml

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

RUN  cp -rf templates/ ./target/x86_64-unknown-linux-musl/release/. \
  && cp -rf blogpost/ ./target/x86_64-unknown-linux-musl/release/. \
  && cp -rf assets/ ./target/x86_64-unknown-linux-musl/release/. \
  && cp -rf sitemap.xml ./target/x86_64-unknown-linux-musl/release/.

## our final base
FROM scratch
WORKDIR /blog
EXPOSE 8080
COPY --from=build /blog/target/x86_64-unknown-linux-musl/release/ .
CMD ["./blog"]
