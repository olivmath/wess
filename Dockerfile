FROM rust as planner
WORKDIR /wess
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cacher
WORKDIR /wess
RUN cargo install cargo-chef
COPY --from=planner /wess/recipe.json recipe.json
RUN apt update && apt install libclang-dev -y
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR /wess
COPY . .
COPY --from=cacher /wess/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN apt update && apt install libclang-dev -y
RUN cargo build -r

FROM gcr.io/distroless/cc-debian11
WORKDIR /wess
COPY --from=builder /wess/target/release/wess /wess
COPY wess.toml wess.yaml ./
EXPOSE 7770
CMD [ "./wess" ]