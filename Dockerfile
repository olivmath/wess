FROM rust

WORKDIR /usr/wess/

COPY Cargo.toml Cargo.lock ./
COPY wess.toml wess.toml
COPY src src

RUN cargo fetch
RUN cargo build -r

EXPOSE 7770