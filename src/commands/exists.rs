// src/commands/exists.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Exists {
    pub keys: Vec<String>,
}

impl Command for Exists {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.exists(self.keys.clone())
    }
}
