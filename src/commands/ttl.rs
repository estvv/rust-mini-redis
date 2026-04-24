// src/commands/ttl.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Ttl {
    pub key: String,
}

impl Command for Ttl {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.ttl(self.key.clone())
    }
}
