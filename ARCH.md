# Documentação do Projeto Wess

## Tabela de Tópicos

1. [Conceitual e Abstrato](#1-conceitual-e-abstrato)  
   1.1. [Introdução](#11-introdução)  
   1.2. [Problema e Solução](#12-problema-e-solução)  

2. [Arquitetura](#2-arquitetura)  
   2.1. [Visão Geral da Arquitetura](#21-visão-geral-da-arquitetura)  
   2.2. [Diagrama de Componentes](#22-diagrama-de-componentes)  
   2.3. [Módulos Principais](#23-módulos-principais)  
   2.4. [Boas Práticas e Clean Code](#24-boas-práticas-e-clean-code)  
   2.5. [Dependências Externas e Monitoramento](#25-dependências-externas-e-monitoramento)  

3. [Tecido (Detalhamento)](#3-tecido-detalhamento)  
   3.1. [Fluxos Principais](#31-fluxos-principais)  
   &nbsp;&nbsp;&nbsp;&nbsp;3.1.1. [Criação de Módulo Wasm](#311-criação-de-módulo-wasm)  
   &nbsp;&nbsp;&nbsp;&nbsp;3.1.2. [Execução de Módulo Wasm](#312-execução-de-módulo-wasm)  
   &nbsp;&nbsp;&nbsp;&nbsp;3.1.3. [Atualização/Exclusão](#313-atualizaçãoexclusão)  
   3.2. [Padrões de Comunicação e Sistema de Cache](#32-padrões-de-comunicação-e-sistema-de-cache)  
   3.3. [Considerações de Segurança](#33-considerações-de-segurança)  
   3.4. [Estratégia de Testes](#34-estratégia-de-testes)  
   3.5. [Estratégia de Deployment e Roadmap Evolutivo](#35-estratégia-de-deployment-e-roadmap-evolutivo)  
   3.6. [API HTTP](#36-api-http)  
   3.7. [Configuração e Variáveis de Ambiente](#37-configuração-e-variáveis-de-ambiente)  
   3.8. [Tratamento de Erros](#38-tratamento-de-erros)  
   3.9. [Limitações, Restrições e Troubleshooting](#39-limitações-restrições-e-troubleshooting)

---

## 1. Conceitual e Abstrato

### 1.1. Introdução

O **Projeto Wess** é uma solução desenvolvida em Rust que visa fornecer uma plataforma robusta para a execução de módulos WebAssembly (Wasm). Ele integra componentes de alta performance e segurança para gerenciar a criação, leitura, atualização, exclusão e execução de módulos Wasm de forma assíncrona.

### 1.2. Problema e Solução

**Problema:**  
Gerenciar a execução isolada de módulos Wasm, garantindo alta performance, segurança e escalabilidade, além de manter a consistência dos dados e fornecer monitoramento e métricas precisas.

**Solução:**  
O sistema é composto por:

- Um **servidor HTTP** com endpoints para operações CRUD e execução de módulos.
- Um conjunto de **workers assíncronos** (Writer, Reader e Runner) que segregam as responsabilidades de escrita, leitura e execução.
- Armazenamento persistente com **RocksDB** e mecanismos de **cache** para otimização.
- Integração com **Prometheus** e **Grafana** para monitoramento e visualização de métricas.
- Um ambiente de testes robusto (BDD com Behave e testes unitários/integrados) e um pipeline **CI/CD** com GitHub Actions.

---

## 2. Arquitetura

### 2.1. Visão Geral da Arquitetura

O sistema Wess implementa uma arquitetura modular e assíncrona, onde cada componente é responsável por uma parte específica da lógica de negócio. A separação de operações entre escrita, leitura e execução permite maior escalabilidade e resiliência.

### 2.2. Diagrama de Componentes

```
[Clientes]
    │
    ▼
[Servidor HTTP]───▶[Writer Worker]───▶[RocksDB]
    │                   │
    ├─▶[Reader Worker]◀─┤
    │                   │
    └─▶[Runner Worker]──┘
            │
            ▼
        [Wasm Runtime]
```

### 2.3. Módulos Principais

- **Configuração (`src/config`):**  
  - Carrega configurações via `wess.toml`.
  - Garante tipagem forte e utiliza singleton (via `lazy_static`).

- **Banco de Dados (`src/database`):**  
  - Abstração sobre o RocksDB.
  - Operações CRUD com suporte para múltiplos ambientes.

- **Servidor HTTP (`src/server`):**  
  - Implementa endpoints para operações CRUD e execução de Wasm.
  - Utiliza middleware para coleta de métricas e gerenciamento de estado global.

- **Workers (`src/workers`):**  
  - **Writer:** Responsável por operações assíncronas de escrita.
  - **Reader:** Implementa cache LRU e leitura persistente.
  - **Runner:** Gerencia a execução dos módulos Wasm, com cache de compilação.

- **Logging e Métricas (`src/logger`, `src/metrics`):**  
  - Integração com log4rs para logging estruturado.
  - Exporta métricas para Prometheus (uso de CPU/memória, tempos de operação, etc.).

### 2.4. Boas Práticas e Clean Code

- **Separação de Responsabilidades:** Cada módulo é responsável por uma parte específica da lógica.
- **Comunicação Assíncrona:** Uso de canais (MPSC) para comunicação entre os workers.
- **Tipagem Forte:** Estruturas de dados rigorosamente definidas para reduzir erros.
- **Documentação Interna:** Uso de rustdoc e comentários detalhados.
- **Tratamento Centralizado de Erros:** Enumeração dos tipos de erros para facilitar o debug e a manutenção.
- **Testabilidade:** Abordagem com testes unitários e de integração, utilizando mocks e ambientes isolados.

### 2.5. Dependências Externas e Monitoramento

- **RocksDB:** Para armazenamento persistente.
- **Wasmer:** Runtime para execução de módulos WebAssembly.
- **Tokio:** Runtime assíncrono.
- **Prometheus & Grafana:** Para monitoramento de métricas.
- **Behave:** Framework para testes comportamentais.
- **Docker:** Para isolar ambientes de testes e deployment.

---

## 3. Tecido (Detalhamento)

### 3.1. Fluxos Principais

#### 3.1.1. Criação de Módulo Wasm

1. **Validação:** Verifica a estrutura e o payload JSON.
2. **Geração de Identificador:** Cria um UUIDv4 para o módulo.
3. **Processamento Assíncrono:** Envia para o Writer Worker.
4. **Persistência:** Salva o módulo no RocksDB.
5. **Resposta:** Retorna o UUID gerado.

#### 3.1.2. Execução de Módulo Wasm

1. **Validação:** Confirma a existência do módulo e dos parâmetros.
2. **Leitura:** Busca o módulo via Reader Worker (primeiro na cache, depois no banco).
3. **Compilação e Cache:** Compila o módulo, utilizando cache para otimização.
4. **Execução:** O Runner Worker executa o módulo isoladamente.
5. **Resposta:** Retorna o resultado da execução.

#### 3.1.3. Atualização/Exclusão

1. **Validação:** Confirma a existência do módulo.
2. **Processamento Assíncrono:** Envia a operação para o Writer Worker.
3. **Cache:** Invalidação dos caches relacionados para manter a consistência.

### 3.2. Padrões de Comunicação e Sistema de Cache

- **Comunicação:**  
  - Uso de canais assíncronos (MPSC) para a troca de mensagens entre componentes.
  - Mensagens estruturadas para garantir a correta propagação de eventos.

- **Cache:**  
  - Implementação de cache LRU no Reader.
  - Configuração do tamanho e TTL via `wess.toml`.
  - Métricas para monitorar hit/miss e taxa de expiração.

### 3.3. Considerações de Segurança

- **Validação de Entrada:**  
  - Sanitização de IDs (UUIDv4) e limites de tamanho para payloads (ex.: máximo 2MB para módulos).
  - Tipagem forte para garantir a integridade dos dados.

- **Isolamento de Execução:**  
  - Utilização do Wasmer para sandboxing.
  - Restrições de memória (ex.: 256MB por instância) e timeout (ex.: 5s padrão).
  - Reutilização de instâncias mantendo o isolamento.

### 3.4. Estratégia de Testes

- **Testes Unitários:**  
  - Cobertura superior a 80% utilizando ferramentas como o cargo tarpaulin.
  - Uso de mocks para dependências (banco em memória, runtime Wasm simulado).

- **Testes de Integração:**  
  - Ambientes Docker isolados para simular cenários reais (carga, falhas controladas e consistência ACID).
  - Testes BDD com o Behave para cenários de uso.

### 3.5. Estratégia de Deployment e Roadmap Evolutivo

- **Deployment Local:**  
  - Uso do Docker Compose para orquestrar serviços (Wess, Prometheus, Grafana, Redis opcional).

- **Roadmap Evolutivo:**  
  - **Próximas Etapas:**  
    - Suporte a WASI (WebAssembly System Interface).  
    - Implementação de plugins de autenticação.  
    - Otimizações AOT (Ahead-Of-Time).  
  - **Melhorias Planejadas:**  
    - Suporte cross-language (C/Rust/Go).  
    - Interface administrativa web e sistema de templates de funções.  
  - **Pesquisa & Desenvolvimento:**  
    - Integração com WASM multi-threaded, otimizações com SIMD e suporte a GPU offloading.

### 3.6. API HTTP

#### Endpoints

- **POST /modules**  
  - **Descrição:** Cria um novo módulo Wasm.  
  - **Payload:**

  ```json
    {
      "name": "string",
      "content": "base64_encoded_wasm"
    }
  ```

  - **Resposta:** UUID do módulo criado.

- **GET /modules/{id}**  
  - **Descrição:** Recupera os detalhes de um módulo Wasm.  
  - **Parâmetros:** ID (UUIDv4).  
  - **Resposta:** Dados do módulo.

- **PUT /modules/{id}**  
  - **Descrição:** Atualiza um módulo existente.  
  - **Payload:** Mesmo formato do POST.  
  - **Resposta:** Status da operação.

- **DELETE /modules/{id}**  
  - **Descrição:** Remove um módulo.  
  - **Parâmetros:** ID (UUIDv4).  
  - **Resposta:** Status da operação.

- **POST /modules/{id}/run**  
  - **Descrição:** Executa uma função do módulo.  
  - **Payload:**

  ```json
    {
      "args": ["value1", "value2"]
    }
  ```

  - **Resposta:** Resultado da execução.

#### Códigos de Erro Comuns

- **400:** Payload inválido.  
- **404:** Módulo não encontrado.  
- **408:** Timeout de execução.  
- **413:** Payload muito grande.  
- **500:** Erro interno.

### 3.7. Configuração e Variáveis de Ambiente

#### Arquivo `wess.toml`

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
path = "./data"
max_open_files = 1000

[reader]
cache_size = 100
ttl = 3600

[runner]
timeout = 5000
max_memory = 268435456 # 256MB
```

#### Variáveis de Ambiente

- `WESS_SERVER_HOST`: Host do servidor.  
- `WESS_SERVER_PORT`: Porta do servidor.  
- `WESS_DB_PATH`: Caminho para o banco de dados.  
- `WESS_LOG_LEVEL`: Nível de log.

### 3.8. Tratamento de Erros

#### Tipos de Erro (Exemplo em Rust)

```rust
pub enum WessError {
  Database(String),
  Validation(String),
  Execution(String),
  NotFound(String),
  Internal(String),
}
```

#### Estratégias

1. Propagação dos erros do banco como `WessError::Database`.
2. Validações incorretas retornam `WessError::Validation`.
3. Erros na execução de Wasm são capturados como `WessError::Execution`.
4. Logging estruturado para facilitar o diagnóstico.
5. Coleta de métricas dos erros por tipo.

### 3.9. Limitações, Restrições e Troubleshooting

#### Limitações e Restrições

- **Módulos Wasm:**  
  - Tamanho máximo de 2MB.  
  - Funções exportadas limitadas a operações síncronas.  
  - Tipos suportados: i32, i64, f32, f64.  
  - Limite de memória por instância: 256MB.

- **API:**  
  - Rate limit: 1000 requisições/minuto por IP.  
  - Timeout de execução: 5s.  
  - Máximo de 10 argumentos por execução.  
  - Payload máximo: 2MB.

- **Recursos:**  
  - CPU: 1 core por execução.  
  - Memória: 256MB por instância.  
  - Conexões: até 10.000 simultâneas.

#### Troubleshooting

- **Logs Importantes:**  

```
<timestamp> ERROR database - Failed to write module: <error>
<timestamp> WARN runner - Execution timeout: module=<id>
<timestamp> INFO metrics - Cache hit ratio: 0.85
```

- **Debug:**  
  1. Habilitar logs em nível `debug` para visualizar payloads, tempos de operação e estados do cache.  
  2. Verificar métricas disponíveis em `/metrics` (ex.: `wess_requests_total`, `wess_errors_total`, `wess_execution_time`, `wess_cache_hits`).

- **Problemas Comuns:**  
  - **Timeout de execução:**  
    - Verificar tamanho do módulo e loops infinitos.  
    - Ajustar `runner.timeout` se necessário.
  - **Cache miss alto:**  
    - Considerar aumentar o `reader.cache_size` ou ajustar o TTL.
  - **Erros de memória:**  
    - Monitorar e ajustar o limite de memória configurado.
  - **Latência alta:**  
    - Investigar possíveis gargalos de CPU ou IO, além de revisar a configuração dos workers.
