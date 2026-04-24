// src/commands/incr.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Incr {
    pub key: String,
}

impl Command for Incr {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.incr(self.key.clone())
    }
}
