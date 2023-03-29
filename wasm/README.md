# How to create a Wasm function.

- You need the `cargo-wasi`, instal it with `cargo install cargo-wasi`
- You need also `wasmtime`, install it with `curl https://wasmtime.dev/install.sh -sSf | bash`

## 1 Create your Rust project.

**Run this command in terminal:**

```
cargo new --lib the_answer
```

**This command will create a dir with tree:**

```
the_answer/
├── Cargo.lock
├── Cargo.toml
└── src/
    └── lib.rs
```

## 2 Write your function.

**Edit your Cargo.toml:**

```toml
[package]
name = "the_answer"
version = "0.1.0"
edition = "2021"

[dependencies]

[lib]
crate-type = ["cdylib"]

[profile.release]
lto = true
codegen-units = 1
opt-level = "z"
```

**This is an example of a function that returns the meaning of life, the universe and everything else, and the answer to such existential complexity is simply "42".**

```rust
#![allow(unused)]
fn main() {
    #[no_mangle]
    pub extern "C" fn the_answer() -> i32 {
        42
    }
}
```

## 3 Build your Rust function to `.wasm`.

**Now build your Rust function to Wasm function:**

```shell
cargo wasi build -r
```

**Get your Wasm file on `the_answer/target/wasm32-wasi/release/the_answer.wasm`:**

```
the_answer/
├── src/
│   └── lib.rs
└── target/
    ├── ...
    └── wasm32-wasi/
        ├── ...
        └── release/
            ├── ...
            └── the_answer.wasm
```

## 4 Test it

**You can test your wasm with `wasmtime`.**

```
wasmtime run --invoke the_answer the_answer.wasm
```

## 5 Convert your `.wasm` to bytes.

**Run this command to convert your .wasm to list of bytes in `BYTES_RESULT.txt`:**

```
od -An -v -t uC the_answer.wasm \
| tr -s ' ' \
| tr ' ' ',' \
| tr -d '\n' \
| sed 's/^,//;s/,$//g' > BYTES_RESULT.txt
```

**BYTES_RESULT.txt Should looking like this:**

```
0,97,115,109,1,0,0,0,1,5,1,96,0,1,127,3,2,1,0,5,3,1,0,16,7,23,2,6,109,101,109,111,114,121,2,0,10,116,104,101,95,97,110,115,119,101,114,0,0,10,6,1,4,0,65,42,11,0,35,4,110,97,109,101,1,28,1,0,25,116,104,101,95,97,110,115,119,101,114,46,99,111,109,109,97,110,100,95,101,120,112,111,114,116,0,104,9,112,114,111,100,117,99,101,114,115,2,8,108,97,110,103,117,97,103,101,1,4,82,117,115,116,0,12,112,114,111,99,101,115,115,101,100,45,98,121,3,5,114,117,115,116,99,29,49,46,54,56,46,49,32,40,56,52,54,48,99,97,56,50,51,32,50,48,50,51,45,48,51,45,50,48,41,5,99,108,97,110,103,6,49,53,46,48,46,54,6,119,97,108,114,117,115,6,48,46,49,57,46,48,0,30,15,116,97,114,103,101,116,95,102,101,97,116,117,114,101,115,1,43,11,98,117,108,107,45,109,101,109,111,114,121
```
