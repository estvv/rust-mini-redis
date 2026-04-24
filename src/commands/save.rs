// src/commands/save.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Save {
    pub filename: String,
}

impl Command for Save {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.save(self.filename.clone())
    }
}
