# Use uma imagem base com Rust
FROM rust:1.68 as builder

# Instale as dependências necessárias para buildar o aplicativo Wess
RUN apt-get update && \
    apt-get install -y clang libclang-dev && \
    rm -rf /var/lib/apt/lists/*

# Crie um diretório para a aplicação
WORKDIR /usr/src/wess

# Copie o arquivo Cargo.toml e Cargo.lock para o diretório do aplicativo
COPY Cargo.toml Cargo.lock ./

# Crie um projeto "dummy" para fazer o cache das dependências
RUN mkdir src && \
    echo "fn main() {println!(\"Dummy\");}" > src/main.rs
RUN cargo build --release

# Copie o código fonte do projeto
COPY src src

# Realize a compilação do projeto
RUN cargo build --release

# Inicie a etapa final do Docker para executar o aplicativo
FROM debian:buster-slim

# Instale as dependências necessárias para executar o aplicativo Wess
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates clang libclang-dev && \
    rm -rf /var/lib/apt/lists/*

# Copie o binário compilado do primeiro estágio
COPY --from=builder /usr/src/wess/target/release/wess /usr/local/bin/wess

# Crie um diretório para os arquivos de log
RUN mkdir log

# Exponha a porta 7770 para o servidor Wess
EXPOSE 7770

# Defina o comando de execução padrão
CMD ["wess"]
