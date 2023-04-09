# Wess: A WebAssembly Execution and Storage Service

Wess is a high-level WebAssembly execution and storage service designed to provide
efficient and easy-to-use WebAssembly function execution. The service allows you to
store and run WebAssembly modules with just a few lines of code, abstracting away the
complexities of compilation, linking, and module instantiation.

Key Features:

- **Efficient Execution**: Wess provides a streamlined execution environment for
  running WebAssembly functions, reducing overhead and improving performance.
- **Caching**: Wess leverages a caching system to store frequently accessed data
  in memory, improving the overall performance of the service.
- **Concurrency**: The service is built with concurrency in mind, allowing for
  efficient execution of multiple tasks simultaneously.
- **Storage**: Wess uses RocksDB for storing WebAssembly modules, providing a
  fast and reliable storage solution.

The main components of Wess include:

- `database`: The module responsible for managing data storage using RocksDB.
- `logger`: A utility module for handling logs and terminal output.
- `server`: The module that implements the Wess server and API endpoints.
- `workers`: A set of modules that manage the execution of WebAssembly functions,
  including reader, writer, and runner.

To get started with Wess

### Tech used

- Rust
    - ![Tokio](https://img.shields.io/badge/Tokio-0079AB?style=for-the-badge&logo=Tokio&logoColor=white)
    - ![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=white)
- Python
    - ![Behave](https://img.shields.io/badge/Behave-444444?style=for-the-badge&logo=behave&logoColor=white)
- JavaScript
    - ![k6](https://img.shields.io/badge/k6-FFFFFF?style=for-the-badge&logo=k6&logoColor=black)
- RocksDB
    - ![RocksDB](https://img.shields.io/badge/RocksDB-007ACC?style=for-the-badge&logo=RocksDB&logoColor=white)
- Kubernetes
    - ![Kubernetes](https://img.shields.io/badge/Kubernetes-326CE5?style=for-the-badge&logo=kubernetes&logoColor=white)
- Grafana
    - ![Grafana](https://img.shields.io/badge/Grafana-F46800?style=for-the-badge&logo=grafana&logoColor=white)
- Prometheus
    - ![Prometheus](https://img.shields.io/badge/Prometheus-E6522C?style=for-the-badge&logo=prometheus&logoColor=white)
