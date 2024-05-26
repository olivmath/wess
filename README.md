# Wess: WebAssembly Service System

## tl;dr

**Just for Learn!!!**

- Store your WebAssembly
- Run your WebAssembly

## Road Map

| Feature                                                                   | Status | Priority |
|---------------------------------------------------------------------------|--------|----------|
| Docs about how to execute a function                                      | ⏰      | 🔥       |
| Import another wasm module into wasm functions                            | ⏰      | 😴       |
| Give access to `syscall` into wasm functions                              | ⏰      | 😴       |
| Make wasm functions with persisted memory state (store/retrieve)          | ⏰      | 😴       |
| Run Wasm functions multiple params (Array<T>) -> (Array<T>)               | ⏰      | 😴       |
| CRUD ops Wasm module                                                      | ✅      | ✅        |
| Run Wasm functions without params () -> i32                               | ✅      | ✅        |
| Run Wasm functions with one params (i32) -> i32                           | ✅      | ✅        |
| Run Wasm functions multiple params (i32) -> i32                           | ✅      | ✅        |
| Run Wasm functions multiple params (i32/i64/f32/f64) -> (i32/i64/f32/f64) | ✅      | ✅        |
| Logging module                                                            | ✅      | ✅        |
| Read/Write ops on RocksDB                                                 | ✅      | ✅        |
| Export metrics with Prometheus                                            | ✅      | ✅        |
| Reading metrics with Grafana                                              | ✅      | ✅        |
| Load tests with k6                                                        | ✅      | ✅        |
| Behave tests with Behave                                                  | ✅      | ✅        |
| Wasm Examples                                                             | ✅      | ✅        |

## How to play?

## Start with

```
cargo run
```

## Upload a Wasm file

- You need convert your wasm to bytes array, see how [here](./wasm/README.md)

```
curl -X POST http://localhost:80 -H "Content-Type: application/json" -d '{
  "wasm": [ 0,97,115,109,1,0,0,0,1,5,1,96,0,1,127,3,2,1,0,5,3,1,0,16,7,23,2,6,109,101,109,111,114,121,2,0,10,116,104,101,95,97,110,115,119,101,114,0,0,10,6,1,4,0,65,42,11,0,35,4,110,97,109,101,1,28,1,0,25,116,104,101,95,97,110,115,119,101,114,46,99,111,109,109,97,110,100,95,101,120,112,111,114,116,0,104,9,112,114,111,100,117,99,101,114,115,2,8,108,97,110,103,117,97,103,101,1,4,82,117,115,116,0,12,112,114,111,99,101,115,115,101,100,45,98,121,3,5,114,117,115,116,99,29,49,46,54,56,46,49,32,40,56,52,54,48,99,97,56,50,51,32,50,48,50,51,45,48,51,45,50,48,41,5,99,108,97,110,103,6,49,53,46,48,46,54,6,119,97,108,114,117,115,6,48,46,49,57,46,48,0,30,15,116,97,114,103,101,116,95,102,101,97,116,117,114,101,115,1,43,11,98,117,108,107,45,109,101,109,111,114,121 ],
  "metadata": {
    "functionName": "the_answer",
    "returnType": [
      "I32"
    ],
    "args": []
  }
}'
>>> {"message":{"id":"b38ed019-45be-42c1-9bd8-2b781b91d082"}}
```

- *You can update your wasm using `PUT`*

```
curl -X PUT http://localhost:80/b38ed019-45be-42c1-9bd8-2b781b91d082
  -H "Content-Type: application/json" -d '{
  "wasm": [ 99, 99, 99, 99 ],
  "metadata": {
    "functionName": "new_function",
    "returnType": [
      "I32"
    ],
    "args": []
  }
}'
>>> {"message":{"id":"b38ed019-45be-42c1-9bd8-2b781b91d082"}}
```


### Run your Wasm

```
curl -X POST http://localhost:80/a180761b-c9d5-482f-a4df-680443ee1cf2 -H "Content-Type: application/json" -d '[]'
>>> {"message":[42]}
```


### How it Works?

#### Read Ops

1. receive a `id`
2. validate it (uuid-v4)
3. and ask **Reader** about the wasm module
4. respond 200 | 400 | 404 | 500


#### Write Ops

1. receive a json
2. validate it (wasm module)
3. create a new uuid
4. respond 202 | 400 | 404 | 500
5. **[Async after return]** send the json to **Writer**
6. **[Async after return]** Writer perform op (Create | Update | delete)


#### Run Ops

1. receive a json
2. validate it (params and uuid-v4)
3. retrieve wasm module from Reader
4. send wasm module to Runner to execute wasm function
5. respond 200 | 400 | 404 | 500


### Channels

- Encapsulates validated data (like a DTO) sends it in a queue to the Worker responsible for the process
- The messages sent are called `Job`s:

```rust
pub enum WriteOps {
    Create,
    Update,
    Delete,
}

pub struct WriteJob {
    pub write_req: Option<WasmModule>,
    pub write_type: WriteOps,
    pub id: String,
}
```

```rust
#[derive(Debug)]
pub struct ReadJob {
    pub responder: Sender<ReadResponse>,
    pub id: String,
}
```

```rust
#[derive(Debug)]
pub struct RunJob {
    pub responder: Sender<RunResponse>,
    pub args: Vec<Value>,
    pub id: String,
}
```

### Workers

#### Writer

- Don't respond anything, just write

1. Receive a `WriteJob` by channel
2. Performs op into database (Rocksdb)


#### Reader


- Just read and respond

1. Receive a `ReadJob` by channel
2. Try found `id` in `Cache`
3. Try found `id` in database (Rocksdb)
4. Send `WasmModule` by channel


#### Runner


- Execute function and return the result

1. Receive a `RunJob` by channel
2. Build `Engine` to try exec function
3. Send result of computation by channel to `RunOps`
4. Send result of computation by channel to `Writter` (stateful call) [Not implemented]

