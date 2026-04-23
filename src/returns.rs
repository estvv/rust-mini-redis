// src/returns.rs

use tokio::sync::broadcast;

pub enum Return {
    Ok(String),
    Err(String),
    NotFound(String),
    Subscribe(broadcast::Receiver<String>),
    Unsubscribe,
}
