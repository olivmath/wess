# Desenvolvimento do Projeto Wess

Este documento fornece um guia passo a passo para configurar o ambiente de desenvolvimento para o projeto Wess.

## Pré-requisitos

Antes de começar, você precisará ter as seguintes ferramentas instaladas em seu sistema:

1. **Rust**: O Wess é construído usando Rust. Você pode instalar o Rust usando o [rustup](https://rustup.rs/).

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Após a instalação, adicione o caminho do Rust ao seu `PATH`:

   ```bash
   source $HOME/.cargo/env
   ```

2. **Docker**: O projeto utiliza Docker para facilitar a construção e execução do ambiente. Você pode instalar o Docker seguindo as instruções em [docs.docker.com](https://docs.docker.com/get-docker/).

3. **Python**: O projeto utiliza Python para testes. Você pode instalar o Python através do gerenciador de pacotes de sua distribuição ou [baixando do site oficial](https://www.python.org/downloads/).

4. **Poetry**: Um gerenciador de dependências para Python. Instale o Poetry com o seguinte comando:

   ```bash
   pip install poetry
   ```

## Configuração do Ambiente

1. **Clone o repositório**:

   ```bash
   git clone <URL_DO_REPOSITORIO>
   cd <NOME_DO_REPOSITORIO>
   ```

2. **Instale as dependências do Python**:

   Navegue até o diretório do projeto e execute:

   ```bash
   poetry install
   ```

3. **Configuração do Docker**:

   Certifique-se de que o Docker está em execução. Você pode construir a imagem Docker usando o seguinte comando:

   ```bash
   docker-compose build
   ```

4. **Executar o projeto**:

   Para iniciar o projeto, você pode usar o comando:

   ```bash
   docker-compose up
   ```

   Isso iniciará todos os serviços definidos no arquivo `docker-compose.yml`.

5. **Executar os testes**:

   Para executar os testes do projeto, você pode usar o seguinte comando:

   ```bash
   poetry run behave tests/behave/features
   ```

## Contribuição

Se você deseja contribuir para o projeto, siga as diretrizes de contribuição e faça um fork do repositório. Certifique-se de que suas alterações estão bem testadas e documentadas.

## Problemas Comuns

- **Erro de permissão no Docker**: Se você encontrar problemas de permissão ao executar comandos do Docker, pode ser necessário adicionar seu usuário ao grupo `docker`:

   ```bash
   sudo usermod -aG docker $USER
   ```

   Após isso, faça logout e login novamente.

- **Problemas com dependências do Python**: Se você encontrar problemas ao instalar dependências, verifique se o `pip` e o `poetry` estão atualizados.

## Conclusão

Agora você deve ter um ambiente de desenvolvimento configurado para o projeto Wess. Sinta-se à vontade para explorar o código e contribuir! 