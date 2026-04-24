// src/commands/set.rs

use std::sync::Arc;

use crate::command::Command;
use crate::returns::Return;

pub struct Set {
    pub key: String,
    pub value: String,
    pub expiration: Option<u64>,
}

impl Command for Set {
    fn execute(&self, db: &Arc<crate::db::Db>, _client_id: u64) -> Return {
        db.set(self.key.clone(), self.value.clone(), self.expiration)
    }
}
