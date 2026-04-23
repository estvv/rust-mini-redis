// src/dispatcher.rs

use crate::request::Request;
use crate::returns::Return;
use crate::stock::Stock;

pub struct Dispatcher {
    stock: Stock,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            stock: Stock::new(),
        }
    }

    pub fn dispatch(&mut self, request: Request) -> Return {
        match request {
            Request::Get(key) => self.get(&key).map_or_else(|| Return::NotFound(key), Return::Ok),
            Request::Set(key, value) => Return::Ok(self.set(key, value)),
            Request::Del(key) => self.del(&key).map_or_else(|| Return::NotFound(key), Return::Ok),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.stock.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) -> String {
        self.stock.set(key, value);
        "OK".into()
    }

    pub fn del(&mut self, key: &str) -> Option<String> {
        if !self.stock.get(key).is_some() {
            return None;
        }
        self.stock.del(key);
        Some("OK".into())
    }
}
