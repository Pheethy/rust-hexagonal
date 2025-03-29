FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv /app/target/release/rust-hexagonal /app/app

FROM debian:stable-slim AS runtime
WORKDIR /app
# Install libssl3 to provide libssl.so.3
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/app /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/app"]
