FROM rust as builder
WORKDIR /usr/wess/
RUN apt update && apt install libclang-dev -y
COPY Cargo.toml Cargo.lock wess.toml wess.yaml ./
COPY src src
RUN cargo fetch
RUN cargo build -r

FROM rust:slim
WORKDIR /usr/wess/
COPY --from=builder /usr/wess/target/release/wess /usr/wess/wess.toml /usr/wess/wess.yaml ./
EXPOSE 7770
CMD [ "./wess" ]