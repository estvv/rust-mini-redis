use rust_mini_redis::dispatcher::Dispatcher;
use rust_mini_redis::request::Request;
use rust_mini_redis::returns::Return;

#[test]
fn test_dispatch_get_nonexistent_key() {
    let mut dispatcher = Dispatcher::new();
    let result = dispatcher.dispatch(Request::GET("key".to_string()), 1);
    assert_eq!(result, Return::NotFound("key".to_string()));
}

#[test]
fn test_dispatch_set_and_get() {
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(
        Request::SET {
            key: "key".to_string(),
            value: "value".to_string(),
            expiration: None,
        },
        1,
    );

    let result = dispatcher.dispatch(Request::GET("key".to_string()), 1);
    assert_eq!(result, Return::Ok("value".to_string()));
}

#[test]
fn test_dispatch_set_with_expiration_and_ttl() {
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(
        Request::SET {
            key: "key".to_string(),
            value: "value".to_string(),
            expiration: Some(10),
        },
        1,
    );

    let result = dispatcher.dispatch(Request::TTL("key".to_string()), 1);
    match result {
        Return::Ok(ttl_str) => {
            assert!(ttl_str.ends_with("ms"));
            let ttl_num: u64 = ttl_str.trim_end_matches("ms").parse().unwrap();
            assert!(ttl_num > 9000 && ttl_num <= 10000);
        }
        _ => panic!("Expected Ok with TTL value"),
    }
}

#[test]
fn test_dispatch_del_existing_key() {
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(
        Request::SET {
            key: "key".to_string(),
            value: "value".to_string(),
            expiration: None,
        },
        1,
    );

    let result = dispatcher.dispatch(Request::DEL("key".to_string()), 1);
    assert_eq!(result, Return::Ok("OK".to_string()));

    let get_result = dispatcher.dispatch(Request::GET("key".to_string()), 1);
    assert_eq!(get_result, Return::NotFound("key".to_string()));
}

#[test]
fn test_dispatch_del_nonexistent_key() {
    let mut dispatcher = Dispatcher::new();
    let result = dispatcher.dispatch(Request::DEL("nonexistent".to_string()), 1);
    assert_eq!(result, Return::NotFound("nonexistent".to_string()));
}

#[test]
fn test_dispatch_incr_nonexistent_key() {
    let mut dispatcher = Dispatcher::new();
    let result = dispatcher.dispatch(Request::INCR("counter".to_string()), 1);
    assert_eq!(result, Return::Ok("1".to_string()));
}

#[test]
fn test_dispatch_incr_existing_key() {
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(
        Request::SET {
            key: "counter".to_string(),
            value: "10".to_string(),
            expiration: None,
        },
        1,
    );

    let result = dispatcher.dispatch(Request::INCR("counter".to_string()), 1);
    assert_eq!(result, Return::Ok("11".to_string()));
}

#[test]
fn test_dispatch_decr_nonexistent_key() {
    let mut dispatcher = Dispatcher::new();
    let result = dispatcher.dispatch(Request::DECR("counter".to_string()), 1);
    assert_eq!(result, Return::Ok("-1".to_string()));
}

#[test]
fn test_dispatch_decr_existing_key() {
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(
        Request::SET {
            key: "counter".to_string(),
            value: "10".to_string(),
            expiration: None,
        },
        1,
    );

    let result = dispatcher.dispatch(Request::DECR("counter".to_string()), 1);
    assert_eq!(result, Return::Ok("9".to_string()));
}

#[test]
fn test_dispatch_drop() {
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(
        Request::SET {
            key: "key1".to_string(),
            value: "value1".to_string(),
            expiration: None,
        },
        1,
    );
    dispatcher.dispatch(
        Request::SET {
            key: "key2".to_string(),
            value: "value2".to_string(),
            expiration: None,
        },
        1,
    );

    let result = dispatcher.dispatch(Request::DROP(), 1);
    assert_eq!(result, Return::Ok("OK".to_string()));

    let result1 = dispatcher.dispatch(Request::GET("key1".to_string()), 1);
    assert_eq!(result1, Return::NotFound("key1".to_string()));

    let result2 = dispatcher.dispatch(Request::GET("key2".to_string()), 1);
    assert_eq!(result2, Return::NotFound("key2".to_string()));
}

#[test]
fn test_dispatch_ttl_nonexistent_key() {
    let mut dispatcher = Dispatcher::new();
    let result = dispatcher.dispatch(Request::TTL("key".to_string()), 1);
    assert_eq!(result, Return::NotFound("key".to_string()));
}

#[test]
fn test_dispatch_ttl_key_without_expiration() {
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(
        Request::SET {
            key: "key".to_string(),
            value: "value".to_string(),
            expiration: None,
        },
        1,
    );

    let result = dispatcher.dispatch(Request::TTL("key".to_string()), 1);
    assert_eq!(result, Return::Ok("None".to_string()));
}

#[test]
fn test_dispatch_subscribe() {
    let mut dispatcher = Dispatcher::new();
    let result = dispatcher.dispatch(Request::SUB("news".to_string()), 1);
    match result {
        Return::Subscribe(_) => (),
        _ => panic!("Expected Subscribe variant"),
    }
}

#[test]
fn test_dispatch_unsubscribe_without_subscription() {
    let mut dispatcher = Dispatcher::new();
    let _ = dispatcher.dispatch(Request::SUB("news".to_string()), 1);
    let result = dispatcher.dispatch(Request::UNSUB("news".to_string()), 2);
    match result {
        Return::Err(msg) => assert!(msg.contains("Not subscribed")),
        _ => panic!("Expected Err variant"),
    }
}
