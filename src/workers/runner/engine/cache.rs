//! # The `cache` module provides an in-memory cache for storing frequently accessed data
//!
//! This module contains the following main components:
//!
//! - [`CompiledWasmCache`]: A struct representing the in-memory cache.
//!
//! The `cache` module is designed to improve performance for frequently accessed data
//! by storing it in memory and using a Least Recently Used (LRU) eviction strategy
//! to manage the cache size.
//!
//! The cache is generic and can be used to store any type of data.

use std::collections::{HashMap, VecDeque};
use wasmtime::Engine;
use wasmtime::Error;
use wasmtime::Module;

const CACHE_SIZE: usize = 1024;
type Id = String;

/// An in-memory cache with a Least Recently Used (LRU) eviction strategy.
#[derive(Clone)]
pub struct CompiledWasmCache {
    /// A HashMap to store the cached data.
    cached: HashMap<Id, Module>,
    /// A VecDeque to maintain the order of recently used items.
    queue: VecDeque<Id>,
}

impl CompiledWasmCache {
    /// # Creates a new instance of the [`CompiledWasmCache`] struct.
    ///
    /// ## Returns
    ///
    /// * A new [`CompiledWasmCache`] instance.
    pub fn new() -> Self {
        Self {
            cached: HashMap::new(),
            queue: VecDeque::with_capacity(CACHE_SIZE),
        }
    }

    /// # Retrieves an item from the cache or the data source if not found.
    ///
    /// ## Arguments
    ///
    /// * `id` - The unique identifier of the item.
    /// * `engine` - A [`wasmtime::Engine`] object for running the WebAssembly module.
    /// * `f` - A closure that fetches the item from the data source if not found in the cache.
    ///
    /// ## Returns
    ///
    /// * A [`Result<Module, Error>`] containing the requested item or an error.
    pub fn get<F>(&mut self, id: Id, engine: Engine, f: F) -> Result<Module, Error>
    where
        F: FnOnce() -> Vec<u8>,
    {
        match self.cached.get(&id) {
            Some(module) => Ok(module.clone()),
            None => {
                let new_module = match Module::from_binary(&engine, &f()) {
                    Ok(module) => module,
                    Err(e) => {
                        eprintln!("{}", e);
                        return Err(e);
                    }
                };
                self.put(id, new_module.clone());
                Ok(new_module)
            }
        }
    }

    /// # Inserts an item into the cache.
    ///
    /// ## Arguments
    ///
    /// * `id` - The unique identifier of the item.
    /// * `new_module` - The item to be inserted.
    ///
    /// This method also handles cache eviction based on the LRU strategy.
    pub fn put(&mut self, id: Id, new_module: Module) {
        if self.queue.len() >= CACHE_SIZE {
            if let Some(removed_id) = self.queue.pop_back() {
                self.cached.remove(removed_id.as_str());
            }
        }
        self.queue.push_front(id.clone());
        self.cached.insert(id, new_module);
    }
}
