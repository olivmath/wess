# How to create a Wasm function

## 1 Create your Rust project

**Run this command in terminal:**

```
cargo new --lib my_wasm_function
```

**This command will create a dir with tree:**

```
my_wasm_function/
├── Cargo.lock
├── Cargo.toml
└── src/
    └── lib.rs
```

## 2 Write your function

**Edit your Cargo.toml:**

```toml
[package]
name = "my_wasm_function"
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

**This is a example of function that sum two numbers:**

```rust
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

## 3 Build your Rust function to .wasm

**Now build your Rust function to Wasm function:**

```shell
cargo build --release --target wasm32-unknown-unknown
```

**Get your Wasm file on `my_wasm_function/target/wasm32-unknown-unknown/release/my_wasm_function.wasm`:**

```
my_wasm_function/
├── src/
│   └── lib.rs
└── target/
    ├── ...
    └── wasm32-unknown-unknown/
        ├── ...
        └── release/
            ├── ...
            └── sum.wasm
```

## 4 Convert your .wasm to bytes

**Run this command to convert your .wasm to list of bytes in `BYTES_RESULT.txt`:**

```
od -An -v -t uC YOUR_WASM_FILE.wasm \
| tr -s ' ' \
| tr ' ' ',' \
| tr -d '\n' \
| sed 's/^,//;s/,$//g' > BYTES_RESULT.txt
```

**BYTES_RESULT.txt Should looking like this:**

```
0,97,115,109,1,0,0,0,1,11,2,96,0,1,127,96,2,127,127,1,127,3,3,2,0,1,5,3,1,0,16,6,25,3,127,1,65,128,128,192,0,11,127,0,65,128,128,192,0,11,127,0,65,128,128,192,0,11,7,44,4,6,109,101,109,111,114,121,2,0,4,109,97,105,110,0,1,10,95,95,100,97,116,97,95,101,110,100,3,1,11,95,95,104,101,97,112,95,98,97,115,101,3,2,10,15,2,4,0,65,42,11,8,0,16,128,128,128,128,0,11,0,51,4,110,97,109,101,1,24,2,0,15,95,95,111,114,105,103,105,110,97,108,95,109,97,105,110,1,4,109,97,105,110,7,18,1,0,15,95,95,115,116,97,99,107,95,112,111,105,110,116,101,114,0,77,9,112,114,111,100,117,99,101,114,115,2,8,108,97,110,103,117,97,103,101,1,4,82,117,115,116,0,12,112,114,111,99,101,115,115,101,100,45,98,121,1,5,114,117,115,116,99,29,49,46,54,56,46,49,32,40,56,52,54,48,99,97,56,50,51,32,50,48,50,51,45,48,51,45,50,48,41
```
