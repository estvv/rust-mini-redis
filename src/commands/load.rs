// src/commands/load.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Load {
    pub filename: String,
}

impl Command for Load {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.load(self.filename.clone())
    }
}
