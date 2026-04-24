use rust_mini_redis::db::Db;
use rust_mini_redis::request::Request;
use rust_mini_redis::returns::Return;
use std::sync::Arc;

#[test]
fn test_dispatch_get_nonexistent_key() {
    let db = Arc::new(Db::new());
    let result = Request::GET("key".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::NotFound("key".to_string()));
}

#[test]
fn test_dispatch_set_and_get() {
    let db = Arc::new(Db::new());
    Request::SET {
        key: "key".to_string(),
        value: "value".to_string(),
        expiration: None,
    }
    .into_command()
    .execute(&db, 1);

    let result = Request::GET("key".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::Ok("value".to_string()));
}

#[test]
fn test_dispatch_set_with_expiration_and_ttl() {
    let db = Arc::new(Db::new());
    Request::SET {
        key: "key".to_string(),
        value: "value".to_string(),
        expiration: Some(10),
    }
    .into_command()
    .execute(&db, 1);

    let result = Request::TTL("key".to_string())
        .into_command()
        .execute(&db, 1);
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
    let db = Arc::new(Db::new());
    Request::SET {
        key: "key".to_string(),
        value: "value".to_string(),
        expiration: None,
    }
    .into_command()
    .execute(&db, 1);

    let result = Request::DEL("key".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::Ok("OK".to_string()));

    let get_result = Request::GET("key".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(get_result, Return::NotFound("key".to_string()));
}

#[test]
fn test_dispatch_del_nonexistent_key() {
    let db = Arc::new(Db::new());
    let result = Request::DEL("nonexistent".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::NotFound("nonexistent".to_string()));
}

#[test]
fn test_dispatch_incr_nonexistent_key() {
    let db = Arc::new(Db::new());
    let result = Request::INCR("counter".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::Ok("1".to_string()));
}

#[test]
fn test_dispatch_incr_existing_key() {
    let db = Arc::new(Db::new());
    Request::SET {
        key: "counter".to_string(),
        value: "10".to_string(),
        expiration: None,
    }
    .into_command()
    .execute(&db, 1);

    let result = Request::INCR("counter".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::Ok("11".to_string()));
}

#[test]
fn test_dispatch_decr_nonexistent_key() {
    let db = Arc::new(Db::new());
    let result = Request::DECR("counter".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::Ok("-1".to_string()));
}

#[test]
fn test_dispatch_decr_existing_key() {
    let db = Arc::new(Db::new());
    Request::SET {
        key: "counter".to_string(),
        value: "10".to_string(),
        expiration: None,
    }
    .into_command()
    .execute(&db, 1);

    let result = Request::DECR("counter".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::Ok("9".to_string()));
}

#[test]
fn test_dispatch_drop() {
    let db = Arc::new(Db::new());
    Request::SET {
        key: "key1".to_string(),
        value: "value1".to_string(),
        expiration: None,
    }
    .into_command()
    .execute(&db, 1);
    Request::SET {
        key: "key2".to_string(),
        value: "value2".to_string(),
        expiration: None,
    }
    .into_command()
    .execute(&db, 1);

    let result = Request::DROP().into_command().execute(&db, 1);
    assert_eq!(result, Return::Ok("OK".to_string()));

    let result1 = Request::GET("key1".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result1, Return::NotFound("key1".to_string()));

    let result2 = Request::GET("key2".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result2, Return::NotFound("key2".to_string()));
}

#[test]
fn test_dispatch_ttl_nonexistent_key() {
    let db = Arc::new(Db::new());
    let result = Request::TTL("key".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::NotFound("key".to_string()));
}

#[test]
fn test_dispatch_ttl_key_without_expiration() {
    let db = Arc::new(Db::new());
    Request::SET {
        key: "key".to_string(),
        value: "value".to_string(),
        expiration: None,
    }
    .into_command()
    .execute(&db, 1);

    let result = Request::TTL("key".to_string())
        .into_command()
        .execute(&db, 1);
    assert_eq!(result, Return::Ok("None".to_string()));
}

#[test]
fn test_dispatch_subscribe() {
    let db = Arc::new(Db::new());
    let result = Request::SUB("news".to_string())
        .into_command()
        .execute(&db, 1);
    match result {
        Return::Subscribe(_) => (),
        _ => panic!("Expected Subscribe variant"),
    }
}

#[test]
fn test_dispatch_unsubscribe_without_subscription() {
    let db = Arc::new(Db::new());
    Request::SUB("news".to_string())
        .into_command()
        .execute(&db, 1);
    let result = Request::UNSUB("news".to_string())
        .into_command()
        .execute(&db, 2);
    match result {
        Return::Err(msg) => assert!(msg.contains("Not subscribed")),
        _ => panic!("Expected Err variant"),
    }
}
