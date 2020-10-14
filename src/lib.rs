#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
use std::vec::Vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

mod split_args;

/// Parse the command line.
///
/// This expects a slice of possible options and turns `-option foo` to `[("option", "foo")]`.
/// Only `-key value` options are supported.
///
/// This function errors, if the command line options are not valid, see `ParseError` for details.
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn parse<'a>(cmdline: &'a mut str, options: &[&'a str]) -> Result<Vec<(&'a str, &'a str)>, ParseError<'a>> {
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
                if options.contains(&a) {
                    last = Some(a);
                } else {
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
