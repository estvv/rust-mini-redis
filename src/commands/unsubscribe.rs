// src/commands/unsubscribe.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Unsubscribe {
    pub channel: String,
}

impl Command for Unsubscribe {
    fn execute(&self, db: &Arc<crate::db::Db>, client_id: u64) -> Return {
        db.unsubscribe(self.channel.clone(), client_id)
    }
}
