// src/commands/get.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Get {
    pub key: String,
}

impl Command for Get {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.get(self.key.clone())
    }
}
