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

use core::iter::FusedIterator;

use crate::parse::{Char, Quote, StrChars, StrIndex, StrRange};

/// Splits a cmdline into multiple args.
///
/// See the [module documentation] for more details.
///
/// [module documentation]: index.html
pub struct SplitArgs<'a> {
    iter: StrChars<'a>,
}

impl<'a> SplitArgs<'a> {
    /// Creates from a cmdline.
    ///
    /// See the [module documentation] for more details.
    ///
    /// [module documentation]: index.html
    pub const fn new(cmdline: &'a str) -> Self {
        Self {
            iter: StrChars::new(cmdline),
        }
    }

    /// Get the substring `start..end`.
    ///
    /// # Panics
    ///
    /// If `start..end` is not valid.
    fn get_range(&self, start: StrIndex, end: StrIndex) -> &'a str {
        let range = StrRange { start, end };
        range.get(self.iter.get()).expect("range should be valid")
    }
}

impl<'a> Iterator for SplitArgs<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.iter.peek()?;

            match c {
                Char::Whitespace => {
                    self.iter.advance();
                    continue;
                }

                Char::Letter(_) => {
                    let start = self.iter.pos();
                    self.iter.advance();

                    while let Some(c) = self.iter.peek() {
                        match c {
                            Char::Letter(_) | Char::Quote(_) => {
                                self.iter.advance();
                            }

                            Char::Whitespace => {
                                let end = self.iter.pos();
                                self.iter.advance();

                                // SAFETY: `start` and `end` are obtained via
                                //         the iterator, so they must be valid.
                                return Some(self.get_range(start, end));
                            }
                        }
                    }

                    // SAFETY: `start` was obtained via the iterator, so this
                    //         range must be valid.
                    return Some(self.get_range(start, self.iter.pos()));
                }

                Char::Quote(Quote::Single) => {
                    self.iter.advance();
                    let start = self.iter.pos();

                    while let Some(c) = self.iter.peek() {
                        match c {
                            Char::Letter(_) | Char::Whitespace | Char::Quote(Quote::Double) => {
                                self.iter.advance();
                            }

                            Char::Quote(Quote::Single) => {
                                let end = self.iter.pos();
                                self.iter.advance();

                                // SAFETY: `start` and `end` are obtained via
                                //         the iterator, so they must be valid.
                                return Some(self.get_range(start, end));
                            }
                        }
                    }

                    // SAFETY: `start` was obtained via the iterator, so this
                    //         range must be valid.
                    return Some(self.get_range(start, self.iter.pos()));
                }

                Char::Quote(Quote::Double) => {
                    self.iter.advance();
                    let start = self.iter.pos();

                    while let Some(c) = self.iter.peek() {
                        match c {
                            Char::Letter(_) | Char::Whitespace | Char::Quote(Quote::Single) => {
                                self.iter.advance();
                            }

                            Char::Quote(Quote::Double) => {
                                let end = self.iter.pos();
                                self.iter.advance();

                                // SAFETY: `start` and `end` are obtained via
                                //         the iterator, so they must be valid.
                                return Some(self.get_range(start, end));
                            }
                        }
                    }

                    // SAFETY: `start` was obtained via the iterator, so this
                    //         range must be valid.
                    return Some(self.get_range(start, self.iter.pos()));
                }
            }
        }
    }
}

impl FusedIterator for SplitArgs<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($test:ident: $cmdline:expr => [ $($arg:expr),* ]) => {
            #[test]
            fn $test() {
                let mut parsed = SplitArgs::new($cmdline);
                $(
                    assert_eq!(parsed.next(), Some($arg));
                )*
                assert_eq!(parsed.next(), None);
            }
        };
    }

    test!(basic: "string" => ["string"]);
    test!(two: "string1 string2" => ["string1", "string2"]);
    test!(single_quotes: "string1 'string2 string3' string4" => ["string1", "string2 string3", "string4"]);
    test!(double_quotes: "string1 \"string2 string3\" string4" => ["string1", "string2 string3", "string4"]);
    test!(single_quotes_two: "'1 2' '3 4'" => ["1 2", "3 4"]);
    test!(double_quotes_two: "\"1 2\" \"3 4\"" => ["1 2", "3 4"]);
    test!(unterminated_single_quotes: "1 \"2 3 4" => ["1", "2 3 4"]);
    test!(unterminated_double_quotes: "1 '2 3 4" => ["1", "2 3 4"]);
    test!(other_whitespace: "1\t2\n3 4\r5" => ["1", "2", "3", "4", "5"]);
    test!(non_ascii: "string1 rusty🦀 party🎉time string2" => ["string1", "rusty🦀", "party🎉time", "string2"]);
    test!(non_ascii_basic: "strÄng" => ["strÄng"]);
    test!(non_ascii_two: "sträng1 sträng2" => ["sträng1", "sträng2"]);
    test!(non_acsii_quotes: "\"sträng1 sträng2\"" => ["sträng1 sträng2"]);
}
