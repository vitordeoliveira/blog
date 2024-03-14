---
filename: "how-host-a-rust-server-in-gcp"
title: "How host a rustlang server with your own DNS for free in the google cloud plataform?"
subtitle: "finding one of the cheapest and easiest ways of host your rust server"
description: "In the Rust language journey, after learning the basics and being
able to create a server, the second thought comes to mind... how do I do it
available for the internet, how should I configure my server for the internet
and what is the cheapest way to do it, if possible for free?"
tags: ["rust", "GCP", "free", "axum"]
similar_posts: [""]
date: "2021-09-13t03:48:00"
finished: true
---

# How host a rustlang server with your own DNS for free in the google cloud plataform?

In the Rust language journey, after learning the basics and being able to
create a server, the second thought comes to mind... how do I do it available
for the internet, how should I configure my server for the internet and what is
the cheapest way to do it, if possible for free?

Here we will answer you that, using the google cloud platform and the cloudrun service.

A little explanation about GCP for the newcomers:

GCP uses the model of account base, and not project base like AWS, so if you
have a gmail account should be very easy to create a GCP account (you might
already have)

Your account on google should be already enough.

Lets create a very simple server in Rust

```bash
cargo new project
cd project/
cargo run
```

That should bring a hello world message.

Now lets create a very simple axum server who simple return a hello world in
the port 8080

for that run

```bash
cargo add axum
cargo add tokio -F full
```

The goal is showing how you post your container in the GCP for free, so I will
not create a fancy server here

```rust
use std::env;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let host = env::var("SERVER_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or("3000".to_string());

    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
```

That is a very simple example, I will enter in details of how create proper
servers with axum in another blog post, if you have any prior experience with
any other programming language, you are able to understand what we are doing
here.

Then after having our service we need to conteinerize him, we use Docker for do this

```Dockerfile
FROM rust:slim-buster as build

# create a new empty shell project
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src


RUN cargo build --release

# # our final base
FROM debian:stable-slim
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
WORKDIR /app

EXPOSE 8080
COPY --from=build /app/target/release/ .
CMD ["./project"]
```

```bash
docker build --platform linux/amd64 -t project .
docker tag project:latest <yourdockerhubaccount>/project:0.0.1
docker push <yourdockerhubaccount>/project:0.0.1
```

note I add **_--plataform linux/amd64_** the reason is because I am in a Macbook, by
default mac build for linux/arm64, that wont work in the cloudrun.

now we need to tag and push to some registry, lets use Dockerhub itself,
because google now is able to pull from there

With docker and the server ready in our side, we are ready to publish to cloud run

- create project
- access cloud run
- setup container
- finished

Now if you want to add a custom DNS, just click here in back, and click in
mapping domain, I will not show this part, but if you want to me create another
post showing, please give me a thumbs up, and give me a comment
