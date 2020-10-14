//! The main file for integration tests.
use miniarg::{parse, ParseError};

#[test]
/// Just calling a binary should produce an empty result.
fn basic() {
    let mut cmdline = "executable".to_string();
    assert_eq!(parse(&mut cmdline, &[]).unwrap(), vec![]);
}

#[test]
/// One key, one value.
fn key_value() {
    let mut cmdline = "executable -key value".to_string();
    assert_eq!(parse(&mut cmdline, &["key"]).unwrap(), vec![("key", "value")]);
}

#[test]
/// two keys, two values.
fn two_key_value() {
    let mut cmdline = "executable -key1 value1 -key2 value2".to_string();
    assert_eq!(
        parse(&mut cmdline, &["key1", "key2"]).unwrap(),
        vec![("key1", "value1"), ("key2", "value2")]
    );
}

#[test]
/// one key, two values.
fn key_two_value() {
    let mut cmdline = "executable -key value1 -key value2".to_string();
    assert_eq!(
        parse(&mut cmdline, &["key", "key"]).unwrap(),
        vec![("key", "value1"), ("key", "value2")]
    );
}

#[test]
/// Just a key should produce an empty vec.
fn value_missing() {
    let mut cmdline = "executable -key".to_string();
    assert_eq!(parse(&mut cmdline, &["key"]).unwrap(), vec![]);
}

#[test]
/// An invalid key should produce an error.
fn invalid_key() {
    let mut cmdline = "executable -invalid".to_string();
    assert_eq!(
        parse(&mut cmdline, &["key"]).unwrap_err(),
        ParseError::UnknownKey("invalid")
    );
}

#[test]
/// An option without a key should produce an error.
fn missing_key() {
    let mut cmdline = "executable value".to_string();
    assert_eq!(
        parse(&mut cmdline, &["key"]).unwrap_err(),
        ParseError::NotAKey("value")
    );
}
