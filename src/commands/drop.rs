// src/commands/drop.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Drop;

impl Command for Drop {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.clear()
    }
}
