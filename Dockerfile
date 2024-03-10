FROM rust:1.76.0 as build

# create a new empty shell project
RUN USER=root cargo new --bin blog
WORKDIR /blog

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./templates/ ./templates/
COPY ./blogpost/ ./blogpost/
COPY ./assets/ ./assets/
COPY ./src ./src


RUN cargo build --release \
  && cp -rf templates/ ./target/release/. \
  && cp -rf blogpost/ ./target/release/. \
  && cp -rf assets/ ./target/release/.

# RUN rm src/*.rs

# copy your source tree

# build for release
# RUN rm ./target/release/deps/blog*
# RUN cargo build --release
#
# # our final base
FROM ubuntu:24.04
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
ENV RUST_LOG=debug
WORKDIR /app

EXPOSE 8080
COPY --from=build /blog/target/release/ .
CMD ["./blog"]
