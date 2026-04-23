// src/channel_manager.rs

use std::collections::HashMap;
use tokio::sync::broadcast;

type ChannelSender = broadcast::Sender<String>;

pub struct ChannelManager {
    channels: HashMap<String, ChannelSender>,
}

impl ChannelManager {
    pub fn new() -> Self {
        ChannelManager {
            channels: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, channel: String) -> broadcast::Receiver<String> {
        let sender = self
            .channels
            .entry(channel.clone())
            .or_insert_with(|| broadcast::channel(16).0);
        sender.subscribe()
    }

    pub fn publish(&self, channel: &str, message: &str) -> Result<usize, String> {
        match self.channels.get(channel) {
            Some(sender) => {
                let full_message = format!("MESSAGE {} {}", channel, message);
                sender
                    .send(full_message)
                    .map_err(|e| format!("Failed to publish: {}", e))
            }
            None => Ok(0),
        }
    }

    pub fn channel_exists(&self, channel: &str) -> bool {
        self.channels.contains_key(channel)
    }
}
