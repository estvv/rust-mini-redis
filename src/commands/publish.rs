// src/commands/publish.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Publish {
    pub channel: String,
    pub message: String,
}

impl Command for Publish {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.publish(self.channel.clone(), self.message.clone())
    }
}
