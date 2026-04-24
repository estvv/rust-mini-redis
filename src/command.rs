// src/command.rs
// Defines the Command trait only - no circular dependency

use crate::returns::Return;
use std::sync::Arc;

pub trait Command {
    fn execute(&self, db: &Arc<crate::db::Db>, client_id: u64) -> Return;
}
