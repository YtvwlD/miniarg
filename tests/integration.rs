//! The main file for integration tests.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg(any(feature = "alloc", feature = "std"))]

extern crate alloc;
use alloc::{vec, vec::Vec};

use miniarg::{parse, ParseError};

#[test]
/// Just calling a binary should produce an empty result.
fn basic() {
    let cmdline = "executable";
    assert_eq!(
        parse::<&str>(&cmdline, &[])
            .collect::<Result<Vec<(_, _)>, _>>()
            .unwrap(),
        Vec::new()
    );
}

#[test]
/// One key, one value.
fn key_value() {
    let cmdline = "executable -key value";
    assert_eq!(
        parse(&cmdline, &["key"])
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![(&"key", "value")]
    );
}

#[test]
/// two keys, two values.
fn two_key_value() {
    let cmdline = "executable -key1 value1 -key2 value2";
    assert_eq!(
        parse(&cmdline, &["key1", "key2"])
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![(&"key1", "value1"), (&"key2", "value2")]
    );
}

#[test]
/// one key, two values.
fn key_two_value() {
    let cmdline = "executable -key value1 -key value2";
    assert_eq!(
        parse(&cmdline, &["key", "key"])
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![(&"key", "value1"), (&"key", "value2")]
    );
}

#[test]
/// Just a key should produce a vec containing the key and an empty string.
// This is used for `-help`.
fn just_key() {
    let cmdline = "executable -key";
    assert_eq!(
        parse(&cmdline, &["key"])
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![(&"key", "")]
    );
}

#[test]
/// An invalid key should produce an error.
fn invalid_key() {
    let cmdline = "executable -invalid";
    assert_eq!(
        parse(&cmdline, &["key"])
            .collect::<Result<Vec<_>, _>>()
            .unwrap_err(),
        ParseError::UnknownKey("invalid")
    );
}

#[test]
/// An option without a key should produce an error.
fn missing_key() {
    let cmdline = "executable value";
    assert_eq!(
        parse(&cmdline, &["key"])
            .collect::<Result<Vec<_>, _>>()
            .unwrap_err(),
        ParseError::NotAKey("value")
    );
}

#[test]
fn non_ascii() {
    let cmdline = "executable -value 🦀🎉";
    assert_eq!(
        parse(&cmdline, &["value"])
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![(&"value", "🦀🎉")]
    );
}
