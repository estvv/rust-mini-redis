// src/commands/decr.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Decr {
    pub key: String,
}

impl Command for Decr {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.decr(self.key.clone())
    }
}
