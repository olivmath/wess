FROM rust as chef
WORKDIR /usr/wess/
COPY Cargo.toml Cargo.lock ./
RUN cargo install cargo-chef


FROM chef as planner
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as cacher
COPY --from=planner /usr/wess/recipe.json recipe.json
RUN apt update && apt install libclang-dev -y
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo fetch
RUN cargo chef cook --release --recipe-path recipe.json


FROM chef as builder 
COPY --from=cacher /usr/wess/target /usr/wess/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN rm -rf src/
COPY src src
RUN cargo build --release --bin wess


FROM gcr.io/distroless/cc-debian11 as runtime
WORKDIR /usr/wess/
COPY --from=builder /usr/wess/target/release/wess .
COPY wess.toml wess.yaml ./
EXPOSE 7770
CMD [ "./wess" ]
