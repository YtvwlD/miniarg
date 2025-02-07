//! Splits a cmdline into multiple args.
//!
//! # Usage
//!
//! ```
//! # use miniarg::split_args::SplitArgs;
//! let mut args = SplitArgs::new("executable param1 \"param2, but with spaces\" param3");
//! assert_eq!(args.next(), Some("executable"));
//! assert_eq!(args.next(), Some("param1"));
//! assert_eq!(args.next(), Some("param2, but with spaces"));
//! assert_eq!(args.next(), Some("param3"));
//! assert_eq!(args.next(), None);
//! ```
//!
//! It never panics or errors.

use crate::str_index::{StrIndex, StrRange};

/// Splits a cmdline into multiple args.
///
/// See the [module documentation] for more details.
///
/// [module documentation]: index.html
pub struct SplitArgs<'a> {
    cmdline: &'a str,
    // Position inside `cmdline`.
    index: StrIndex,
    // Tells us the `start` of the current argument.
    //
    // This field is `None` if we are not currently parsing any argument.
    start: Option<StrIndex>,
    // Tells us whether we are currently trying to parse a quoted argument.
    in_quotes: bool,
}

impl<'a> SplitArgs<'a> {
    /// Creates from a cmdline.
    ///
    /// See the [module documentation] for more details.
    ///
    /// [module documentation]: index.html
    pub fn new(cmdline: &'a str) -> Self {
        Self {
            cmdline,
            index: StrIndex::zero(),
            start: None,
            in_quotes: false,
        }
    }
}

impl<'a> Iterator for SplitArgs<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(curr) = self.index.get(self.cmdline) else {
                // When we finish parsing `cmdline`, we must ensure that we also
                // return the final argument.
                match self.start.take() {
                    Some(start) => return Some(&self.cmdline[start.byte_index()..]),
                    None => return None,
                }
            };

            match curr {
                '\"' => {
                    // `in_quotes` tells us whether we are currently in quotes.
                    // If we are and we encounter a quote, then it must be a
                    // closing quote, otherwise it is an opening quote.
                    if self.in_quotes {
                        let end = self.index;
                        self.index.advance('\"');
                        self.in_quotes = false;

                        // SAFETY: We are parsing a closing quote, `start`
                        //         should have been set when we were parsing
                        //         the opening quote.
                        let start = self
                            .start
                            .take()
                            .expect("start should have been set before");

                        let arg = &self.cmdline[StrRange { start, end }];
                        return Some(arg);
                    } else {
                        match self.start {
                            // We are parsing an opening quote here.
                            None => {
                                self.index.advance('\"');
                                self.in_quotes = true;
                                self.start = Some(self.index);
                            }

                            // If we reach a quote but `start` is already set,
                            // then we are in this case: 'value"string...'. In
                            // this case we are going to return the whole
                            // argument including the quotes in the middle.
                            Some(_) => {
                                self.index.advance('\"');
                            }
                        }
                    }
                }

                // If we encounter a space and we are not currently parsing a quoted
                // string, then we must return an argument.
                ' ' if !self.in_quotes => match self.start {
                    Some(start) => {
                        let end = self.index;
                        self.start = None;
                        self.index.advance(' ');

                        let arg = &self.cmdline[StrRange { start, end }];
                        return Some(arg);
                    }

                    // If `start` has not been set, then we are parsing multiple
                    // whitespace between arguments. Simply ignore it and move on.
                    None => {
                        self.index.advance(' ');
                    }
                },

                // If we encounter any other character, then simply set `start`
                // if it is not already set (this happens when we start parsing
                // the argument after whitespace) and advance the index.
                _ => {
                    if self.start.is_none() {
                        self.start = Some(self.index);
                    }

                    self.index.advance(curr);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// one continuous string
    fn basic() {
        let mut parsed = SplitArgs::new("string");
        assert_eq!(parsed.next(), Some("string"));
        assert_eq!(parsed.next(), None);
    }

    #[test]
    /// two strings
    fn two() {
        let mut parsed = SplitArgs::new("string1 string2");
        assert_eq!(parsed.next(), Some("string1"));
        assert_eq!(parsed.next(), Some("string2"));
        assert_eq!(parsed.next(), None);
    }

    #[test]
    /// one string in quotes
    fn quotes() {
        let mut parsed = SplitArgs::new("\"string1 string2\"");
        assert_eq!(parsed.next(), Some("string1 string2"));
        assert_eq!(parsed.next(), None);
    }

    #[test]
    /// one string in quotes
    fn quotes_two() {
        let mut parsed = SplitArgs::new("\"1 2\" \"3 4\"");
        assert_eq!(parsed.next(), Some("1 2"));
        assert_eq!(parsed.next(), Some("3 4"));
        assert_eq!(parsed.next(), None);
    }

    #[test]
    /// one string in quotes, two without
    fn quotes_no_quotes() {
        let mut parsed = SplitArgs::new("1 \"2 3 4\" 5");
        assert_eq!(parsed.next(), Some("1"));
        assert_eq!(parsed.next(), Some("2 3 4"));
        assert_eq!(parsed.next(), Some("5"));
        assert_eq!(parsed.next(), None);
    }

    #[test]
    fn quotes_in_middle() {
        let mut parsed = SplitArgs::new("1 2\"3\"4 5");
        assert_eq!(parsed.next(), Some("1"));
        assert_eq!(parsed.next(), Some("2\"3\"4"));
        assert_eq!(parsed.next(), Some("5"));
    }

    #[test]
    /// one continuous string
    fn non_ascii_basic() {
        let mut parsed = SplitArgs::new("strÄng");
        assert_eq!(parsed.next(), Some("strÄng"));
        assert_eq!(parsed.next(), None);
    }

    #[test]
    /// two strings
    fn non_ascii_two() {
        let mut parsed = SplitArgs::new("sträng1 sträng2");
        assert_eq!(parsed.next(), Some("sträng1"));
        assert_eq!(parsed.next(), Some("sträng2"));
        assert_eq!(parsed.next(), None);
    }

    #[test]
    /// one string in quotes
    fn non_ascii_quotes() {
        let mut parsed = SplitArgs::new("\"sträng1 sträng2\"");
        assert_eq!(parsed.next(), Some("sträng1 sträng2"));
        assert_eq!(parsed.next(), None);
    }
}
