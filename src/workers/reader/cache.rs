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

use crate::database::models::WasmFn;
use std::collections::{HashMap, VecDeque};

const CACHE_SIZE: usize = 1024;
type Id = String;

/// An in-memory cache with a Least Recently Used (LRU) eviction strategy.
pub struct Cache {
    /// A HashMap to store the cached data.
    cached: HashMap<Id, WasmFn>,
    /// A VecDeque to maintain the order of recently used items.
    queue: VecDeque<Id>,
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
            queue: VecDeque::with_capacity(CACHE_SIZE),
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
    /// * An [`Option<WasmFn>`] containing the requested item or [`None`] if not found.
    pub fn get<F>(&mut self, id: Id, f: F) -> Option<WasmFn>
    where
        F: FnOnce() -> Option<WasmFn>,
    {
        match self.cached.get(id.as_str()) {
            Some(wasm_fn) => Some(wasm_fn.clone()),
            None => match f() {
                Some(new_wasm_fn) => {
                    self.put(id, new_wasm_fn.clone());
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
    pub fn put(&mut self, id: Id, new_wasm_fn: WasmFn) {
        if self.queue.len() >= CACHE_SIZE {
            if let Some(removed_id) = self.queue.pop_back() {
                self.cached.remove(removed_id.as_str());
            }
        }
        self.queue.push_front(id.clone());
        self.cached.insert(id, new_wasm_fn);
    }
}
