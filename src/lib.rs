#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("either `std` or `alloc` feature is currently required to build this crate");

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
use std::{
    string::{String, ToString},
    vec::Vec,
};
#[cfg(feature = "alloc")]
use alloc::{
    vec::Vec,
    string::{String, ToString},
};

mod split_args;

/// Parse the command line.
///
/// This expects a slice of possible options and turns `-option foo` to `[("option", "foo")]`.
/// Only `-key value` options are supported.
///
/// This function errors, if the command line options are not valid, see `ParseError` for details.
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn parse<'a, 'b, T>(cmdline: &'a mut str, options: &'b [T]) -> Result<Vec<(&'b T, &'a str)>, ParseError<'a>>
where T: ToString {
    let args = split_args::SplitArgs::new(cmdline);
    let mut result = Vec::new();
    let mut last = None;
    // skip argv[0]
    for arg in args.skip(1) {
        if let Some(l) = last {
            // the last element was a key
            result.push((l, arg));
            last = None;
        } else {
            // the next element has to be a key
            if let Some(a) = arg.strip_prefix("-") {
                last = options.iter().find(|o| first_lower(&o.to_string()) == a);
                if last.is_none() {
                    return Err(ParseError::UnknownKey(a))
                }
            } else {
                return Err(ParseError::NotAKey(arg))
            }
        }
    }
    Ok(result)
}

#[derive(Debug, PartialEq)]
/// Errors occurred during parsing the command line.
pub enum ParseError<'a> {
    /// expected a key, but argument didn't start with a dash
    NotAKey(&'a str),
    /// key is not accepted
    UnknownKey(&'a str),
    /// the default error
    Unknown,
}

/// The main trait.
///
/// Derive this with an enum to get the functionality.
/// Each kind represents an "-{key}" option.
/// They all have a string as a value and may occur multiple times.
#[cfg(feature = "derive")]
pub trait Key {
    /// Parse the cmdline.
    ///
    /// You'll get a vector containing tuples with two strings or with an enum kind and a string.
    #[cfg(any(feature = "alloc", feature = "std"))]
    fn parse<T>(cmdline: &mut str) -> Result<Vec<(&T, &str)>, ParseError>;
}

#[cfg(feature = "derive")]
pub use miniarg_derive::Key;

/// Turn the first character into lowercase.
pub(crate) fn first_lower(input: &str) -> String {
    // taken from https://stackoverflow.com/a/38406885/2192464
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}
