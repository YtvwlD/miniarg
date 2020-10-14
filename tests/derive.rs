//! Tests for the derive macro.
#![cfg(feature = "derive")]

#[cfg(feature = "std")]
use std::fmt;
#[cfg(not(feature = "std"))]
use core::fmt;

use miniarg::{Key, ParseError};

#[derive(Debug, Key, PartialEq)]
enum SimpleKeys {
    Key,
    Key1,
    Key2,
}

#[test]
/// Just calling a binary should produce an empty result.
fn basic() {
    let mut cmdline = "executable".to_string();
    assert_eq!(SimpleKeys::parse(&mut cmdline).unwrap(), Vec::new());
}

#[test]
/// One key, one value.
fn key_value() {
    let mut cmdline = "executable -key value".to_string();
    assert_eq!(SimpleKeys::parse(&mut cmdline).unwrap(), vec![(&SimpleKeys::Key, "value")]);
}

#[test]
/// two keys, two values.
fn two_key_value() {
    let mut cmdline = "executable -key1 value1 -key2 value2".to_string();
    assert_eq!(
        SimpleKeys::parse(&mut cmdline).unwrap(),
        vec![(&SimpleKeys::Key1, "value1"), (&SimpleKeys::Key2, "value2")]
    );
}

#[test]
/// one key, two values.
fn key_two_value() {
    let mut cmdline = "executable -key value1 -key value2".to_string();
    assert_eq!(
        SimpleKeys::parse(&mut cmdline).unwrap(),
        vec![(&SimpleKeys::Key, "value1"), (&SimpleKeys::Key, "value2")]
    );
}

#[test]
/// Just a key should produce an empty vec.
fn value_missing() {
    let mut cmdline = "executable -key".to_string();
    assert_eq!(SimpleKeys::parse(&mut cmdline).unwrap(), vec![]);
}

#[test]
/// An invalid key should produce an error.
fn invalid_key() {
    let mut cmdline = "executable -invalid".to_string();
    assert_eq!(
        SimpleKeys::parse(&mut cmdline).unwrap_err(),
        ParseError::UnknownKey("invalid")
    );
}

#[test]
/// An option without a key should produce an error.
fn missing_key() {
    let mut cmdline = "executable value".to_string();
    assert_eq!(
        SimpleKeys::parse(&mut cmdline).unwrap_err(),
        ParseError::NotAKey("value")
    );
}
