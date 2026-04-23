// src/stock.rs

use std::collections::HashMap;

pub struct Stock {
    map: HashMap<String, String>,
}

impl Stock {
    pub fn new() -> Self {
        Stock {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn del(&mut self, key: &str) {
        self.map.remove(key);
    }
}
