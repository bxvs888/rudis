use redis::Commands;
use redis::cmd;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_unique_suffix() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}_{}", now.as_secs(), now.subsec_nanos())
}

#[tokio::test]
async fn test_bitop_and() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    let unique_suffix = get_unique_suffix();
    let key1 = format!("test_bitop_key1_{}", unique_suffix);
    let key2 = format!("test_bitop_key2_{}", unique_suffix);
    let dest = format!("test_bitop_dest_{}", unique_suffix);

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg(&key1).arg(&key2).arg(&dest).query(&mut con).unwrap_or(());

    // set two keys with known bit patterns
    // 'a' = 0x61 = 01100001
    // 'b' = 0x62 = 01100010
    let _: () = con.set(&key1, "a").unwrap();
    let _: () = con.set(&key2, "b").unwrap();

    // perform BITOP AND
    let result: i64 = cmd("BITOP")
        .arg("AND")
        .arg(&dest)
        .arg(&key1)
        .arg(&key2)
        .query(&mut con)
        .unwrap();
    assert!(result > 0);

    // verify the result exists
    let value: Option<String> = con.get(&dest).unwrap();
    assert!(value.is_some());
}

#[tokio::test]
async fn test_bitop_or() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    let unique_suffix = get_unique_suffix();
    let key1 = format!("test_bitop_or_key1_{}", unique_suffix);
    let key2 = format!("test_bitop_or_key2_{}", unique_suffix);
    let dest = format!("test_bitop_or_dest_{}", unique_suffix);

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg(&key1).arg(&key2).arg(&dest).query(&mut con).unwrap_or(());

    // set two keys
    let _: () = con.set(&key1, "a").unwrap();
    let _: () = con.set(&key2, "b").unwrap();

    // perform BITOP OR
    let result: i64 = cmd("BITOP")
        .arg("OR")
        .arg(&dest)
        .arg(&key1)
        .arg(&key2)
        .query(&mut con)
        .unwrap();
    assert!(result > 0);

    // verify the result exists
    let value: Option<String> = con.get(&dest).unwrap();
    assert!(value.is_some());
}

#[tokio::test]
async fn test_bitop_xor() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    let unique_suffix = get_unique_suffix();
    let key1 = format!("test_bitop_xor_key1_{}", unique_suffix);
    let key2 = format!("test_bitop_xor_key2_{}", unique_suffix);
    let dest = format!("test_bitop_xor_dest_{}", unique_suffix);

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg(&key1).arg(&key2).arg(&dest).query(&mut con).unwrap_or(());

    // set two keys
    let _: () = con.set(&key1, "a").unwrap();
    let _: () = con.set(&key2, "b").unwrap();

    // perform BITOP XOR
    let result: i64 = cmd("BITOP")
        .arg("XOR")
        .arg(&dest)
        .arg(&key1)
        .arg(&key2)
        .query(&mut con)
        .unwrap();
    assert!(result > 0);

    // verify the result exists
    let value: Option<String> = con.get(&dest).unwrap();
    assert!(value.is_some());
}

#[tokio::test]
async fn test_bitop_not() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    let unique_suffix = get_unique_suffix();
    let key = format!("test_bitop_not_key_{}", unique_suffix);
    let dest = format!("test_bitop_not_dest_{}", unique_suffix);

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg(&key).arg(&dest).query(&mut con).unwrap_or(());

    // set a key
    let _: () = con.set(&key, "a").unwrap();

    // perform BITOP NOT
    let result: i64 = cmd("BITOP")
        .arg("NOT")
        .arg(&dest)
        .arg(&key)
        .query(&mut con)
        .unwrap();
    assert!(result > 0);

    // verify the result exists - use get_bytes to handle binary data
    let value: Option<Vec<u8>> = con.get(&dest).unwrap();
    assert!(value.is_some());
    assert!(!value.unwrap().is_empty());
}

#[tokio::test]
async fn test_bitop_multiple_keys() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    let unique_suffix = get_unique_suffix();
    let key1 = format!("test_bitop_multi_key1_{}", unique_suffix);
    let key2 = format!("test_bitop_multi_key2_{}", unique_suffix);
    let key3 = format!("test_bitop_multi_key3_{}", unique_suffix);
    let dest = format!("test_bitop_multi_dest_{}", unique_suffix);

    // clean up any keys that may be present
    let _: () = cmd("DEL")
        .arg(&key1)
        .arg(&key2)
        .arg(&key3)
        .arg(&dest)
        .query(&mut con)
        .unwrap_or(());

    // set multiple keys
    let _: () = con.set(&key1, "a").unwrap();
    let _: () = con.set(&key2, "b").unwrap();
    let _: () = con.set(&key3, "c").unwrap();

    // perform BITOP AND with multiple keys
    let result: i64 = cmd("BITOP")
        .arg("AND")
        .arg(&dest)
        .arg(&key1)
        .arg(&key2)
        .arg(&key3)
        .query(&mut con)
        .unwrap();
    assert!(result > 0);

    // verify the result exists
    let value: Option<String> = con.get(&dest).unwrap();
    assert!(value.is_some());
}

#[tokio::test]
async fn test_bitop_with_nonexistent_keys() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    let unique_suffix = get_unique_suffix();
    let dest = format!("test_bitop_nonexistent_dest_{}", unique_suffix);

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg(&dest).query(&mut con).unwrap_or(());

    // perform BITOP with non-existent keys (should treat as empty strings)
    let result: i64 = cmd("BITOP")
        .arg("AND")
        .arg(&dest)
        .arg("nonexistent_key1")
        .arg("nonexistent_key2")
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 0); // AND of empty strings is empty

    // verify the result is empty or doesn't exist
    let value: Option<String> = con.get(&dest).unwrap();
    // Result should be empty string or None
    assert!(value.is_none() || value.unwrap().is_empty());
}

#[tokio::test]
async fn test_bitop_different_lengths() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    let unique_suffix = get_unique_suffix();
    let key1 = format!("test_bitop_len_key1_{}", unique_suffix);
    let key2 = format!("test_bitop_len_key2_{}", unique_suffix);
    let dest = format!("test_bitop_len_dest_{}", unique_suffix);

    // clean up any keys that may be present
    let _: () = cmd("DEL").arg(&key1).arg(&key2).arg(&dest).query(&mut con).unwrap_or(());

    // set keys with different lengths
    let _: () = con.set(&key1, "a").unwrap(); // 1 byte
    let _: () = con.set(&key2, "foobar").unwrap(); // 6 bytes

    // perform BITOP OR (result should be length of longest)
    let result: i64 = cmd("BITOP")
        .arg("OR")
        .arg(&dest)
        .arg(&key1)
        .arg(&key2)
        .query(&mut con)
        .unwrap();
    assert_eq!(result, 6); // Should be length of longest string

    // verify the result
    let value: Option<String> = con.get(&dest).unwrap();
    assert!(value.is_some());
    assert_eq!(value.unwrap().len(), 6);
}