use rust_mini_redis::channel_manager::ChannelManager;

#[test]
fn test_new_channel_manager_is_empty() {
    let manager = ChannelManager::new();
    assert!(!manager.channel_exists("nonexistent"));
}

#[test]
fn test_subscribe_creates_new_channel() {
    let mut manager = ChannelManager::new();

    assert!(!manager.channel_exists("news"));

    let _receiver = manager.subscribe("news".to_string());

    assert!(manager.channel_exists("news"));
}

#[test]
fn test_subscribe_to_existing_channel() {
    let mut manager = ChannelManager::new();

    let _receiver1 = manager.subscribe("news".to_string());
    let _receiver2 = manager.subscribe("news".to_string());

    assert!(manager.channel_exists("news"));
}

#[test]
fn test_subscribe_multiple_channels() {
    let mut manager = ChannelManager::new();

    let _receiver1 = manager.subscribe("news".to_string());
    let _receiver2 = manager.subscribe("sports".to_string());
    let _receiver3 = manager.subscribe("tech".to_string());

    assert!(manager.channel_exists("news"));
    assert!(manager.channel_exists("sports"));
    assert!(manager.channel_exists("tech"));
}

#[test]
fn test_publish_to_nonexistent_channel() {
    let manager = ChannelManager::new();

    let result = manager.publish("nonexistent", "message");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_channel_exists_false_for_nonexistent() {
    let manager = ChannelManager::new();

    assert!(!manager.channel_exists("nonexistent"));
}

#[test]
fn test_channel_exists_true_after_subscribe() {
    let mut manager = ChannelManager::new();
    let _receiver = manager.subscribe("news".to_string());

    assert!(manager.channel_exists("news"));
}
