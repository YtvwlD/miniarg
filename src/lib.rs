#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use core::iter::Skip;
#[cfg(any(feature = "alloc", feature = "std"))]
use alloc::{
    string::{String, ToString},
};

use cfg_if::cfg_if;

mod split_args;
use split_args::SplitArgs;

// This is a bit of a hack to allow building without std and without alloc.
#[cfg(not(any(feature = "alloc", feature = "std")))]
pub trait ToString {
    fn to_string(&self) -> &str;
}
#[cfg(not(any(feature = "alloc", feature = "std")))]
impl<'b> ToString for &str {
    fn to_string(&self) -> &str {
        self
    }
}

/// Parse the command line.
///
/// This expects a slice of possible options and turns `-option foo` to `[("option", "foo")]`.
/// Only `-key value` options are supported.
///
/// This function errors, if the command line options are not valid, see `ParseError` for details.
pub fn parse<'a, 'b, T>(cmdline: &'a str, options: &'b [T]) -> ArgumentIterator<'a, 'b, T>
where T: ToString {
    let args = SplitArgs::new(cmdline);
    ArgumentIterator::<'a, 'b, T>::new(args, options)
}

pub struct ArgumentIterator<'a, 'b, T> where T: ToString {
    args: Skip<SplitArgs<'a>>,
    options: &'b [T],
    last: Option<&'b T>,
}

impl<'a, 'b, T> ArgumentIterator<'a, 'b, T> where T: ToString {
    fn new(args: SplitArgs<'a>, options: &'b [T]) -> Self {
        // skip argv[0]
        ArgumentIterator {args: args.skip(1), options, last: None}
    }
    
}

impl<'a, 'b, T> Iterator for ArgumentIterator<'a, 'b, T> where T: ToString {
    type Item = Result<(&'b T, &'a str), ParseError<'a>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let arg = match self.args.next() {
                Some(a) => a,
                None => return None,
            };
            if let Some(l) = self.last {
                // the last element was a key
                self.last = None;
                return Some(Ok((l, arg)));
            } else {
                // the next element has to be a key
                if let Some(a) = arg.strip_prefix("-") {
                    self.last = self.options.iter().find(|o| {
                        cfg_if! {
                            if #[cfg(any(feature = "alloc", feature = "std"))] {
                                first_lower(&o.to_string())
                            } else {
                                o.to_string()
                            }
                        }
                    } == a);
                    if self.last.is_none() {
                        return Some(Err(ParseError::UnknownKey(a)))
                    }
                } else {
                    return Some(Err(ParseError::NotAKey(arg)))
                }
            }
        }
    }
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

#[cfg(all(feature = "derive", not(any(feature = "alloc", feature = "std"))))]
compile_error!("either `std` or `alloc` feature is currently required to get the derive feature");


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
    fn parse(cmdline: &str) -> ArgumentIterator<Self> where Self: ToString + Sized;
}

#[cfg(feature = "derive")]
pub use miniarg_derive::Key;

/// Turn the first character into lowercase.
#[cfg(any(feature = "alloc", feature = "std"))]
fn first_lower(input: &str) -> String {
    // taken from https://stackoverflow.com/a/38406885/2192464
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}
