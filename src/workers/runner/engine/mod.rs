use std::fs::read;

use wasmtime::{Engine, Error, Linker, Module, Store, WasmResults};

use crate::database::models::WasmFn;

pub struct Runtime {
    wasm_fn: WasmFn,
}

impl Runtime {
    pub fn new(wasm_fn: WasmFn) -> Self {
        Self { wasm_fn }
    }

    pub fn run(&mut self) -> wasmtime::Result<i32, Error> {
        let wasm_file = get_wasm_file()?;
        let engine = Engine::default();
        let module = match Module::from_binary(&engine, &wasm_file) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}", format!("🚨 Erro ao criar modulo :: {e}"));
                return Err(e);
            }
        };
        let linker = Linker::new(&engine);
        let mut storage = Store::new(&engine, ());
        let instance = match linker.instantiate(&mut storage, &module) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("{}", format!("🚨 Erro ao criar instancia :: {e}"));
                return Err(e);
            }
        };
        let function = match instance.get_typed_func::<(), i32>(&mut storage, "main") {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "🚨 Erro ao recuperar function :: {} - {}",
                    &self.wasm_fn.metadata.func, e
                );
                return Err(e);
            }
        };
        let result: i32 = match function.call(storage, ()) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}", format!("🚨 Erro ao executar function :: {e}"));
                return Err(e);
            }
        };

        Ok(result)
    }
}

fn get_wasm_file() -> Result<Vec<u8>, std::io::Error> {
    let path = "./wasm/forty-two/forty_two.wasm";
    match read(path) {
        Ok(w) => Ok(w),
        Err(e) => {
            eprintln!("🚨 Erro ao ler .wasm :: {} - {}", path, e);
            Err(e)
        }
    }
}
