use rust_mini_redis::stock::Stock;
use std::thread;
use std::time::Duration;

#[test]
fn test_new_stock_is_empty() {
    let mut stock = Stock::new();
    assert!(stock.get(&"nonexistent".to_string()).is_none());
}

#[test]
fn test_set_and_get() {
    let mut stock = Stock::new();
    stock.set("key1".to_string(), "value1".to_string());

    let result = stock.get(&"key1".to_string());
    assert_eq!(result, Some(&"value1".to_string()));
}

#[test]
fn test_get_nonexistent_key() {
    let mut stock = Stock::new();
    let result = stock.get(&"nonexistent".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_set_overwrites_existing_key() {
    let mut stock = Stock::new();
    stock.set("key".to_string(), "value1".to_string());
    stock.set("key".to_string(), "value2".to_string());

    let result = stock.get(&"key".to_string());
    assert_eq!(result, Some(&"value2".to_string()));
}

#[test]
fn test_del_existing_key() {
    let mut stock = Stock::new();
    stock.set("key".to_string(), "value".to_string());

    let result = stock.del(&"key".to_string());
    assert_eq!(result, Some("value".to_string()));

    let get_result = stock.get(&"key".to_string());
    assert_eq!(get_result, None);
}

#[test]
fn test_del_nonexistent_key() {
    let mut stock = Stock::new();
    let result = stock.del(&"nonexistent".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_set_with_expiration() {
    let mut stock = Stock::new();
    stock.set_with_expiration("key".to_string(), "value".to_string(), 1);

    let result = stock.get(&"key".to_string());
    assert_eq!(result, Some(&"value".to_string()));

    let ttl_result = stock.ttl("key".to_string());
    assert!(ttl_result.is_some());
    assert!(ttl_result.unwrap() >= 900 && ttl_result.unwrap() <= 1000);
}

#[test]
fn test_expiration_key_is_removed_after_expiry() {
    let mut stock = Stock::new();
    stock.set_with_expiration("key".to_string(), "value".to_string(), 1);

    thread::sleep(Duration::from_millis(1100));

    let result = stock.get(&"key".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_ttl_with_nonexistent_key() {
    let mut stock = Stock::new();
    let result = stock.ttl("nonexistent".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_ttl_with_no_expiration() {
    let mut stock = Stock::new();
    stock.set("key".to_string(), "value".to_string());

    let result = stock.ttl("key".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_incr_nonexistent_key_starts_at_1() {
    let mut stock = Stock::new();
    let result = stock.incr("counter".to_string());

    assert_eq!(result, Ok(1));
    assert_eq!(stock.get(&"counter".to_string()), Some(&"1".to_string()));
}

#[test]
fn test_incr_increments_existing_value() {
    let mut stock = Stock::new();
    stock.set("counter".to_string(), "10".to_string());

    let result = stock.incr("counter".to_string());
    assert_eq!(result, Ok(11));
    assert_eq!(stock.get(&"counter".to_string()), Some(&"11".to_string()));
}

#[test]
fn test_incr_on_non_numeric_value_resets_to_1() {
    let mut stock = Stock::new();
    stock.set("key".to_string(), "not_a_number".to_string());

    let result = stock.incr("key".to_string());
    assert_eq!(result, Ok(1));
    assert_eq!(stock.get(&"key".to_string()), Some(&"1".to_string()));
}

#[test]
fn test_decr_nonexistent_key_starts_at_minus_1() {
    let mut stock = Stock::new();
    let result = stock.decr("counter".to_string());

    assert_eq!(result, Ok(-1));
    assert_eq!(stock.get(&"counter".to_string()), Some(&"-1".to_string()));
}

#[test]
fn test_decr_decrements_existing_value() {
    let mut stock = Stock::new();
    stock.set("counter".to_string(), "10".to_string());

    let result = stock.decr("counter".to_string());
    assert_eq!(result, Ok(9));
    assert_eq!(stock.get(&"counter".to_string()), Some(&"9".to_string()));
}

#[test]
fn test_drop_clears_all_keys() {
    let mut stock = Stock::new();
    stock.set("key1".to_string(), "value1".to_string());
    stock.set("key2".to_string(), "value2".to_string());

    stock.drop();

    assert!(stock.get(&"key1".to_string()).is_none());
    assert!(stock.get(&"key2".to_string()).is_none());
}

#[test]
fn test_multiple_operations_sequence() {
    let mut stock = Stock::new();

    stock.set("a".to_string(), "1".to_string());
    assert_eq!(stock.get(&"a".to_string()), Some(&"1".to_string()));

    stock.incr("a".to_string()).unwrap();
    assert_eq!(stock.get(&"a".to_string()), Some(&"2".to_string()));

    stock.del(&"a".to_string());
    assert_eq!(stock.get(&"a".to_string()), None);
}

#[test]
fn test_incr_on_expired_key_returns_1() {
    let mut stock = Stock::new();
    stock.set_with_expiration("key".to_string(), "5".to_string(), 1);

    thread::sleep(Duration::from_millis(1100));

    let result = stock.incr("key".to_string());
    assert_eq!(result, Ok(1));
}

#[test]
fn test_decr_on_expired_key_returns_minus_1() {
    let mut stock = Stock::new();
    stock.set_with_expiration("key".to_string(), "5".to_string(), 1);

    thread::sleep(Duration::from_millis(1100));

    let result = stock.decr("key".to_string());
    assert_eq!(result, Ok(-1));
}
