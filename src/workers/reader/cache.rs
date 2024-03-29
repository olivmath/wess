//! # The `cache` module provides an in-memory cache for storing frequently accessed data
//!
//! This module contains the following main components:
//!
//! - [`Cache`]: A struct representing the in-memory cache.
//!
//! The `cache` module is designed to improve performance for frequently accessed data
//! by storing it in memory and using a Least Recently Used (LRU) eviction strategy
//! to manage the cache size.
//!
//! The cache is generic and can be used to store any type of data.

use crate::{config::CONFIG, database::models::WasmModule};
use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};

lazy_static! {
    static ref CACHE_SIZE: usize = CONFIG.reader.cache_size;
}

/// An in-memory cache with a Least Recently Used (LRU) eviction strategy.
pub struct Cache {
    /// A HashMap to store the cached data.
    cached: HashMap<String, WasmModule>,
    /// A VecDeque to maintain the order of recently used items.
    queue: VecDeque<String>,
}

impl Cache {
    /// # Creates a new instance of the [`Cache`] struct.
    ///
    /// ## Returns
    ///
    /// * A new [`Cache`] instance.
    pub fn new() -> Self {
        Self {
            cached: HashMap::new(),
            queue: VecDeque::with_capacity(*CACHE_SIZE),
        }
    }

    /// # Retrieves an item from the cache or the data source if not found.
    ///
    /// ## Arguments
    ///
    /// * `id` - The unique identifier of the item.
    /// * `f` - A closure that fetches the item from the data source if not found in the cache.
    ///
    /// ## Returns
    ///
    /// * An [`Option<WasmModule>`] containing the requested item or [`None`] if not found.
    pub fn get<F>(&mut self, id: &str, f: F) -> Option<WasmModule>
    where
        F: Fn(&str) -> Option<WasmModule>,
    {
        match self.cached.get(id) {
            Some(wasm_module) => Some(wasm_module.clone()),
            None => match f(id) {
                Some(new_wasm_fn) => {
                    self.put(id.to_string(), new_wasm_fn.clone());
                    Some(new_wasm_fn)
                }
                None => return None,
            },
        }
    }

    /// # Inserts an item into the cache.
    ///
    /// ## Arguments
    ///
    /// * `id` - The unique identifier of the item.
    /// * `new_wasm_fn` - The item to be inserted.
    ///
    /// This method also handles cache eviction based on the LRU strategy.
    pub fn put(&mut self, id: String, new_wasm_fn: WasmModule) {
        if self.queue.len() >= *CACHE_SIZE {
            if let Some(removed_id) = self.queue.pop_back() {
                self.cached.remove(removed_id.as_str());
            }
        }
        self.queue.push_front(id.clone());
        self.cached.insert(id, new_wasm_fn);
    }

    pub fn del(&mut self, id: String) {
        self.cached.remove(id.as_str());
        self.queue.retain(|key| key != id.as_str());
    }
}
