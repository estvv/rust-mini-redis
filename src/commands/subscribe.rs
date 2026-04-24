// src/commands/subscribe.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Subscribe {
    pub channel: String,
}

impl Command for Subscribe {
    fn execute(&self, db: &Arc<crate::db::Db>, client_id: u64) -> Return {
        db.subscribe(self.channel.clone(), client_id)
    }
}
