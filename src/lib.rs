//! A minimal argument parser, with support for no-std and no-alloc
//!
//! Only cmdlines in the form of `program -foo value -bar value` are supported.
//! (That means: values are strings, keys start with a single dash, keys can occur multiple times.)
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! miniarg = "0.2"
//! ```
//! The feature `std` is enabled by default and `alloc` and `derive` are optional.
//!
//! # Examples
//!
//! A minimal example looks like this:
//! ```
//! let cmdline = "executable -key value";
//! let mut args = miniarg::parse(&cmdline, &["key"]);
//! assert_eq!(args.next(), Some(Ok((&"key", "value"))));
//! assert_eq!(args.next(), None);
//! ```
//!
//! If you don't want to pass a cmdline, you can use an iterator instead:
//!
//! ```
//! let iter = vec!["executable", "-key", "value"].into_iter();
//! let mut args = miniarg::parse_from_iter(iter, &["key"]);
//! assert_eq!(args.next(), Some(Ok((&"key", "value"))));
//! assert_eq!(args.next(), None);
//! ```
//!
//! You can use `collect::<Result<Vec<_>, _>>()` to get a `Vec`:
//! ```
//! let cmdline = "executable -key value";
//! let args = miniarg::parse(&cmdline, &["key"]).collect::<Result<Vec<_>, _>>()?;
//! assert_eq!(args, vec![(&"key", "value")]);
//! # Ok::<(), miniarg::ParseError<'static>>(())
//! ```
//!
//! If you compile with `std` or `alloc`, it also supports passing [`ToString`] instead of strings,
//! for example your own enum:
//! ```
//! #[derive(Debug, PartialEq)]
//! enum MyKeys {
//!     Foo,
//!     Bar,
//! }
//! impl std::fmt::Display for MyKeys {
//!     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//!         std::fmt::Debug::fmt(self, f)
//!     }
//! }
//! let cmdline = "executable -foo value -bar value";
//! let args = miniarg::parse(&cmdline, &[MyKeys::Foo, MyKeys::Bar])
//! .collect::<Result<Vec<_>, _>>()?;
//! assert_eq!(args, vec![(&MyKeys::Foo, "value"), (&MyKeys::Bar, "value")]);
//! # Ok::<(), miniarg::ParseError<'static>>(())
//! ```
//! As you can see, the first character of the enum kinds is converted to lowercase.
//!
//! If you compile with `derive`, you can use a custom derive instead:
//! ```ignore
//! #[derive(Debug, Key, PartialEq)]
//! enum MyKeys {
//!     Foo,
//!     Bar,
//! }
//! let cmdline = "executable -foo value -bar value";
//! let args = MyKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>()?;
//! assert_eq!(args, vec![(&MyKeys::Foo, "value"), (&MyKeys::Bar, "value")]);
//! # Ok::<(), miniarg::ParseError<'static>>(())
//! ```
//!
//! In this case a help text is generated from the documentation comments on your enum kinds,
//! `help_text()` retrieves it.
//!
//! The code never panics, but the returned iterator will contain [`ParseError`]s
//! if anything goes wrong.
//!
//! You might also want to take a look at the [`split_args`] module for lower level access.
//!
//! [`ToString`]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html
//! [`ParseError`]: enum.ParseError.html
//! [`split_args`]: split_args/index.html
#![doc(html_root_url = "https://docs.rs/miniarg/0.2.0")]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;
use core::iter::Skip;
use core::fmt;
#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
#[cfg(feature = "std")]
use std::error::Error;

use cfg_if::cfg_if;

pub mod split_args;
use split_args::SplitArgs;

// This is a bit of a hack to allow building without std and without alloc.
#[cfg(not(feature = "alloc"))]
pub trait ToString {
    fn to_string(&self) -> &str;
}
#[cfg(not(feature = "alloc"))]
impl<'b> ToString for &str {
    fn to_string(&self) -> &str {
        self
    }
}
#[cfg(not(feature = "std"))]
trait Error {}

/// Parse the command line.
///
/// See the main crate documentation for more details and examples.
pub fn parse<'a, 'b, T>(
    cmdline: &'a str, options: &'b [T]
) -> ArgumentIterator<'a, 'b, T, SplitArgs<'a>>
where T: ToString {
    let args = SplitArgs::new(cmdline);
    ArgumentIterator::<'a, 'b, T, SplitArgs>::new(args, options)
}

/// Parse from a custom iterator.
///
/// It's like [`parse`] but instead of taking a string and splitting it using [`SplitArgs`]
/// it takes the options from a custom iterator.
///
/// See the main crate documentation for more details and examples.
///
/// [`parse`]: fn.parse.html
/// [`SplitArgs`]: split_args/struct.SplitArgs.html
pub fn parse_from_iter<'a, 'b, T, S>(args: S, options: &'b [T]) -> ArgumentIterator<'a, 'b, T, S>
where T: ToString, S: Iterator<Item = &'a str> {
    ArgumentIterator::<'a, 'b, T, S>::new(args, options)
}

/// The iterator returned by [`parse`] and [`parse_from_iter`].
///
/// [`parse`]: fn.parse.html
/// [`parse_from_iter`]: fn.parse_from_iter.html
pub struct ArgumentIterator<'a, 'b, T, S> where T: ToString, S: Iterator<Item = &'a str> {
    args: Skip<S>,
    options: &'b [T],
    last: Option<&'b T>,
}

impl<'a, 'b, T, S> ArgumentIterator<'a, 'b, T, S> where T: ToString, S: Iterator<Item = &'a str> {
    fn new(args: S, options: &'b [T]) -> Self {
        // skip argv[0]
        ArgumentIterator {args: args.skip(1), options, last: None}
    }
    
}

impl<'a, 'b, T, S> Iterator for ArgumentIterator<'a, 'b, T, S>
where T: ToString, S: Iterator<Item = &'a str> {
    type Item = Result<(&'b T, &'a str), ParseError<'a>>;
    
    /// Get the next key pair or an error.
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[non_exhaustive]
/// Errors occurred during parsing the command line.
pub enum ParseError<'a> {
    /// expected a key, but argument didn't start with a dash
    NotAKey(&'a str),
    /// key is not accepted
    UnknownKey(&'a str),
    // the default error
    _Unknown,
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::NotAKey(s) => write!(f, "expected '{}' to start with a dash", s),
            Self::UnknownKey(s) => write!(f, "'{}' is not a known key", s),
            _ => write!(f, "unknown parse error"),
        }
    }
}
impl<'a> Error for ParseError<'a> {}

#[cfg(all(feature = "derive", not(feature = "alloc")))]
compile_error!("at least the `alloc` feature is currently required to get the derive feature");


/// The main trait.
///
/// Derive this with an enum to get the functionality.
/// Each kind represents a `-key value` option (starts with lowercase).
/// They all have a string as a value and may occur multiple times.
///
/// The crate needs to be compiled with `derive` and either `std` or `alloc`.
///
/// # Example
/// ```
/// # #[macro_use] use miniarg::*;
/// use std::fmt;
/// #[derive(Debug, Key, PartialEq, Eq, Hash)]
/// enum MyKeys {
///     Foo,
///     Bar,
/// }
/// # fn main() -> Result<(), miniarg::ParseError<'static>> {
/// let cmdline = "executable -foo value -bar value";
/// let args = MyKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>()?;
/// assert_eq!(args, vec![(&MyKeys::Foo, "value"), (&MyKeys::Bar, "value")]);
/// # Ok(())
/// # }
#[cfg(feature = "derive")]
pub trait Key {
    /// Parse the cmdline.
    ///
    /// You'll get an iterator yielding key value pairs.
    fn parse(cmdline: &str) -> ArgumentIterator<Self, SplitArgs> where Self: ToString + Sized;
    
    /// Get a help text.
    ///
    /// This is being created from the enum kinds and their documentation comments.
    fn help_text() -> &'static str;
}

/// custom derive for the [`Key`] trait
///
/// [`Key`]: trait.Key.html
#[cfg(feature = "derive")]
pub use miniarg_derive::Key;

/// Turn the first character into lowercase.
#[cfg(feature = "alloc")]
fn first_lower(input: &str) -> String {
    // taken from https://stackoverflow.com/a/38406885/2192464
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}
