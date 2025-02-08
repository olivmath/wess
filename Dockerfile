# Stage 1: Planner - Prepare dependencies
FROM rust:latest AS planner
WORKDIR /wess
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Cacher - Cache dependencies
FROM rust:latest AS cacher
WORKDIR /wess
RUN cargo install cargo-chef
COPY --from=planner /wess/recipe.json recipe.json
RUN apt-get update && \
    apt-get install -y libclang-dev && \
    rm -rf /var/lib/apt/lists/*
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3: Builder - Build the application
FROM rust:latest AS builder
WORKDIR /wess
COPY . .
COPY --from=cacher /wess/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN apt-get update && \
    apt-get install -y libclang-dev && \
    rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# Stage 4: Runtime - Create final image
FROM ubuntu:latest
WORKDIR /wess
COPY --from=builder /wess/target/release/wess /wess
COPY wess.toml wess.yaml ./
EXPOSE 7770
CMD ["./wess"]