//! Integration tests for the no alloc case.
//! These are almost the same as main file, but without `collect`.
#![no_std]
use miniarg::{parse, ParseError};

#[test]
/// Just calling a binary should produce an empty result.
fn basic() {
    let cmdline = "executable";
    assert_eq!(parse::<&str>(&cmdline, &[]).next(), None);
}

#[test]
/// One key, one value.
fn key_value() {
    let cmdline = "executable -key value";
    let mut iter = parse(&cmdline, &["key"]);
    assert_eq!(iter.next(), Some(Ok((&"key", "value"))));
    assert_eq!(iter.next(), None);
}

#[test]
/// two keys, two values.
fn two_key_value() {
    let cmdline = "executable -key1 value1 -key2 value2";
    let mut iter = parse(&cmdline, &["key1", "key2"]);
    assert_eq!(iter.next(), Some(Ok((&"key1", "value1"))));
    assert_eq!(iter.next(), Some(Ok((&"key2", "value2"))));
    assert_eq!(iter.next(), None);
}

#[test]
/// one key, two values.
fn key_two_value() {
    let cmdline = "executable -key value1 -key value2";
    let mut iter = parse(&cmdline, &["key", "key"]);
    assert_eq!(iter.next(), Some(Ok((&"key", "value1"))));
    assert_eq!(iter.next(), Some(Ok((&"key", "value2"))));
    assert_eq!(iter.next(), None);
}

#[test]
/// Just a key should produce an empty vec.
fn value_missing() {
    let cmdline = "executable -key";
    assert_eq!(parse(&cmdline, &["key"]).next(), None);
}

#[test]
/// An invalid key should produce an error.
fn invalid_key() {
    let cmdline = "executable -invalid";
    assert_eq!(
        parse(&cmdline, &["key"]).next().unwrap().unwrap_err(),
        ParseError::UnknownKey("invalid")
    );
}

#[test]
/// An option without a key should produce an error.
fn missing_key() {
    let cmdline = "executable value";
    assert_eq!(
        parse(&cmdline, &["key"]).next().unwrap().unwrap_err(),
        ParseError::NotAKey("value")
    );
}
