use rust_mini_redis::request::Request;

#[test]
fn test_parse_get_valid() {
    let result = Request::parse("GET mykey").unwrap();
    assert_eq!(result, Request::GET("mykey".to_string()));
}

#[test]
fn test_parse_get_case_insensitive() {
    let result = Request::parse("get mykey").unwrap();
    assert_eq!(result, Request::GET("mykey".to_string()));

    let result = Request::parse("GeT mykey").unwrap();
    assert_eq!(result, Request::GET("mykey".to_string()));
}

#[test]
fn test_parse_get_too_few_args() {
    let result = Request::parse("GET");
    assert!(result.is_err());
}

#[test]
fn test_parse_get_too_many_args() {
    let result = Request::parse("GET key1 key2");
    assert!(result.is_err());
}

#[test]
fn test_parse_set_valid() {
    let result = Request::parse("SET mykey myvalue").unwrap();
    assert_eq!(
        result,
        Request::SET {
            key: "mykey".to_string(),
            value: "myvalue".to_string(),
            expiration: None
        }
    );
}

#[test]
fn test_parse_set_with_spaces_in_value() {
    let result = Request::parse("SET mykey value with spaces").unwrap();
    assert_eq!(
        result,
        Request::SET {
            key: "mykey".to_string(),
            value: "value with spaces".to_string(),
            expiration: None
        }
    );
}

#[test]
fn test_parse_set_with_expiration() {
    let result = Request::parse("SET mykey myvalue EXP 100").unwrap();
    assert_eq!(
        result,
        Request::SET {
            key: "mykey".to_string(),
            value: "myvalue".to_string(),
            expiration: Some(100)
        }
    );
}

#[test]
fn test_parse_set_expiration_case_insensitive() {
    let result = Request::parse("SET mykey value exp 50").unwrap();
    assert_eq!(
        result,
        Request::SET {
            key: "mykey".to_string(),
            value: "value".to_string(),
            expiration: Some(50)
        }
    );
}

#[test]
fn test_parse_set_with_expiration_and_spaces_in_value() {
    let result = Request::parse("SET mykey value with spaces EXP 200").unwrap();
    assert_eq!(
        result,
        Request::SET {
            key: "mykey".to_string(),
            value: "value with spaces".to_string(),
            expiration: Some(200)
        }
    );
}

#[test]
fn test_parse_set_too_few_args() {
    let result = Request::parse("SET key");
    assert!(result.is_err());
}

#[test]
fn test_parse_set_exp_before_key_and_value() {
    let result = Request::parse("SET EXP 100 value");
    assert!(result.is_err());
}

#[test]
fn test_parse_set_exp_without_value() {
    let result = Request::parse("SET key value EXP");
    assert!(result.is_err());
}

#[test]
fn test_parse_set_invalid_expiration_value() {
    let result = Request::parse("SET key value EXP notanumber");
    assert!(result.is_err());
}

#[test]
fn test_parse_del_valid() {
    let result = Request::parse("DEL mykey").unwrap();
    assert_eq!(result, Request::DEL("mykey".to_string()));
}

#[test]
fn test_parse_del_too_few_args() {
    let result = Request::parse("DEL");
    assert!(result.is_err());
}

#[test]
fn test_parse_del_too_many_args() {
    let result = Request::parse("DEL key1 key2");
    assert!(result.is_err());
}

#[test]
fn test_parse_incr_valid() {
    let result = Request::parse("INCR counter").unwrap();
    assert_eq!(result, Request::INCR("counter".to_string()));
}

#[test]
fn test_parse_incr_case_insensitive() {
    let result = Request::parse("incr counter").unwrap();
    assert_eq!(result, Request::INCR("counter".to_string()));
}

#[test]
fn test_parse_incr_too_few_args() {
    let result = Request::parse("INCR");
    assert!(result.is_err());
}

#[test]
fn test_parse_incr_too_many_args() {
    let result = Request::parse("INCR key1 key2");
    assert!(result.is_err());
}

#[test]
fn test_parse_decr_valid() {
    let result = Request::parse("DECR counter").unwrap();
    assert_eq!(result, Request::DECR("counter".to_string()));
}

#[test]
fn test_parse_decr_too_few_args() {
    let result = Request::parse("DECR");
    assert!(result.is_err());
}

#[test]
fn test_parse_save_valid() {
    let result = Request::parse("SAVE data.json").unwrap();
    assert_eq!(result, Request::SAVE("data.json".to_string()));
}

#[test]
fn test_parse_save_without_json_extension() {
    let result = Request::parse("SAVE data.txt");
    assert!(result.is_err());
}

#[test]
fn test_parse_save_too_few_args() {
    let result = Request::parse("SAVE");
    assert!(result.is_err());
}

#[test]
fn test_parse_load_valid() {
    let result = Request::parse("LOAD data.json").unwrap();
    assert_eq!(result, Request::LOAD("data.json".to_string()));
}

#[test]
fn test_parse_load_without_json_extension() {
    let result = Request::parse("LOAD data.txt");
    assert!(result.is_err());
}

#[test]
fn test_parse_load_too_few_args() {
    let result = Request::parse("LOAD");
    assert!(result.is_err());
}

#[test]
fn test_parse_drop_valid() {
    let result = Request::parse("DROP").unwrap();
    assert_eq!(result, Request::DROP());
}

#[test]
fn test_parse_drop_case_insensitive() {
    let result = Request::parse("drop").unwrap();
    assert_eq!(result, Request::DROP());
}

#[test]
fn test_parse_pub_valid() {
    let result = Request::parse("PUB news Hello World").unwrap();
    assert_eq!(
        result,
        Request::PUB {
            channel: "news".to_string(),
            message: "Hello World".to_string()
        }
    );
}

#[test]
fn test_parse_pub_too_few_args() {
    let result = Request::parse("PUB channel");
    assert!(result.is_err());
}

#[test]
fn test_parse_sub_valid() {
    let result = Request::parse("SUB news").unwrap();
    assert_eq!(result, Request::SUB("news".to_string()));
}

#[test]
fn test_parse_sub_too_few_args() {
    let result = Request::parse("SUB");
    assert!(result.is_err());
}

#[test]
fn test_parse_sub_too_many_args() {
    let result = Request::parse("SUB channel1 channel2");
    assert!(result.is_err());
}

#[test]
fn test_parse_unsub_valid() {
    let result = Request::parse("UNSUB news").unwrap();
    assert_eq!(result, Request::UNSUB("news".to_string()));
}

#[test]
fn test_parse_unsub_too_few_args() {
    let result = Request::parse("UNSUB");
    assert!(result.is_err());
}

#[test]
fn test_parse_ttl_valid() {
    let result = Request::parse("TTL mykey").unwrap();
    assert_eq!(result, Request::TTL("mykey".to_string()));
}

#[test]
fn test_parse_ttl_case_insensitive() {
    let result = Request::parse("ttl mykey").unwrap();
    assert_eq!(result, Request::TTL("mykey".to_string()));
}

#[test]
fn test_parse_ttl_too_few_args() {
    let result = Request::parse("TTL");
    assert!(result.is_err());
}

#[test]
fn test_parse_ttl_too_many_args() {
    let result = Request::parse("TTL key1 key2");
    assert!(result.is_err());
}

#[test]
fn test_parse_unknown_command() {
    let result = Request::parse("UNKNOWN arg1 arg2");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_string() {
    let result = Request::parse("");
    assert!(result.is_err());
}

#[test]
fn test_parse_whitespace_only() {
    let result = Request::parse("   ");
    assert!(result.is_err());
}

#[test]
fn test_parse_with_leading_trailing_whitespace() {
    let result = Request::parse("  GET mykey  ").unwrap();
    assert_eq!(result, Request::GET("mykey".to_string()));
}

#[test]
fn test_get_callback_valid() {
    let parts = vec!["GET", "mykey"];
    let result = Request::get_callback(parts).unwrap();
    assert_eq!(result, Request::GET("mykey".to_string()));
}

#[test]
fn test_set_callback_valid() {
    let parts = vec!["SET", "key", "value"];
    let result = Request::set_callback(parts).unwrap();
    assert_eq!(
        result,
        Request::SET {
            key: "key".to_string(),
            value: "value".to_string(),
            expiration: None
        }
    );
}

#[test]
fn test_set_callback_with_expiration() {
    let parts = vec!["SET", "key", "value", "EXP", "100"];
    let result = Request::set_callback(parts).unwrap();
    assert_eq!(
        result,
        Request::SET {
            key: "key".to_string(),
            value: "value".to_string(),
            expiration: Some(100)
        }
    );
}
