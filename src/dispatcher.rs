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
            Request::Get(key) => self.get(key),
            Request::Set { key, value, expiration } => self.set(key, value, expiration),
            Request::Del(key) => self.del(key),
            Request::Save(filename) => self.save(filename),
            Request::Load(filename) => self.load(filename),
            Request::Drop() => self.drop(),
        }
    }

    pub fn get(&mut self, key: String) -> Return {
        if self.stock.get(&key).is_none() {
            return Return::NotFound(key.to_string());
        }
        Return::Ok(self.stock.get(&key).unwrap().to_string())
    }

    pub fn set(&mut self, key: String, value: String, expiration: Option<u64>) -> Return {
        if expiration.is_none() {
            self.stock.set(key, value);
            return Return::Ok("OK".into());
        }
        self.stock
            .set_with_expiration(key, value, expiration.unwrap());
        Return::Ok("OK".into())
    }

    pub fn del(&mut self, key: String) -> Return {
        if !self.stock.get(&key).is_some() {
            return Return::NotFound(key.to_string());
        }
        self.stock.del(&key);
        Return::Ok("OK".into())
    }

    pub fn save(&mut self, filename: String) -> Return {
        match self.stock.save(&filename) {
            Ok(msg) => Return::Ok(msg),
            Err(e) => Return::Err(e),
        }
    }

    pub fn load(&mut self, filename: String) -> Return {
        match self.stock.load(&filename) {
            Ok(msg) => Return::Ok(msg),
            Err(e) => Return::Err(e),
        }
    }

    pub fn drop(&mut self) -> Return {
        self.stock.drop();
        Return::Ok("OK".into())
    }
}
