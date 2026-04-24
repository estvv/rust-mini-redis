// src/commands/del.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Del {
    pub key: String,
}

impl Command for Del {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.del(self.key.clone())
    }
}
