use core::fmt;
use core::iter::FusedIterator;
use core::ops::Index;

/// A safe index into [`str`] that works over codepoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrIndex(usize);

impl StrIndex {
    /// Create a new [`StrIndex`] that points at the start of a [`str`].
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Get the byte index.
    ///
    /// This can be used to safely index into a [`str`].
    pub const fn byte_index(self) -> usize {
        self.0
    }

    /// Increment the index with `c`.
    ///
    /// This method ensures that the index is always pointing at a [`char`]
    /// boundary.
    pub const fn advance(&mut self, c: char) {
        self.0 += c.len_utf8();
    }

    /// Get the [`char`] from `s`.
    pub fn get(self, s: &str) -> Option<char> {
        let s = s.get(self.0..)?;
        s.chars().next()
    }
}

/// A [`Range`] but for [`str`].
///
/// Semantics are equivalent to [`Range`].
///
/// [`Range`]: core::ops::Range
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrRange {
    pub start: StrIndex,
    pub end: StrIndex,
}

impl StrRange {
    /// Get the [`str`] slice of this range from `s`.
    ///
    /// Returns [`None`] is the range points outside `s`.
    pub fn get(self, s: &str) -> Option<&str> {
        let start = self.start.byte_index();
        let end = self.end.byte_index();

        s.get(start..end)
    }
}

impl Index<StrRange> for str {
    type Output = Self;

    fn index(&self, index: StrRange) -> &Self::Output {
        let start = index.start.byte_index();
        let end = index.end.byte_index();

        &self[start..end]
    }
}

/// Types of quotes.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quote {
    /// '
    Single,
    /// "
    Double,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Char {
    /// Any [`char`]s which are classified as "whitespace".
    ///
    /// See: [`char::is_whitespace`].
    Whitespace,
    /// Any [`char`] that is neither whitespace nor a quote.
    ///
    /// This contains 'A'..'Z', 'a'..'z', '0'..'9', symbols and all other such
    /// UTF-8 characters
    Letter(char),
    /// ' and "
    Quote(Quote),
}

impl fmt::Debug for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Whitespace => write!(f, "\" \""),
            Self::Letter(c) => write!(f, "\"{c}\""),
            Self::Quote(Quote::Single) => write!(f, "\"'\""),
            Self::Quote(Quote::Double) => write!(f, "\"\"\""),
        }
    }
}

impl From<char> for Char {
    fn from(c: char) -> Self {
        if c.is_whitespace() {
            Self::Whitespace
        } else if c == '\'' {
            Self::Quote(Quote::Single)
        } else if c == '\"' {
            Self::Quote(Quote::Double)
        } else {
            Self::Letter(c)
        }
    }
}

/// An [`Iterator`] over the [`char`] of a [`str`] that provides peeking and
/// UTF-8-safe access to the underlying [`str`].
#[derive(Debug, Clone)]
pub struct StrChars<'a> {
    s: &'a str,
    pos: StrIndex,
}

impl<'a> StrChars<'a> {
    /// Create a new [`StrChars`].
    pub const fn new(s: &'a str) -> Self {
        Self {
            s,
            pos: StrIndex::zero(),
        }
    }

    /// Get the underlying string.
    pub const fn get(&self) -> &'a str {
        self.s
    }

    /// Get the position of the next codepoint in the string.
    ///
    /// The returned [`StrIndex`] is always valid.
    pub const fn pos(&self) -> StrIndex {
        self.pos
    }

    /// Peek at the next codepoint.
    ///
    /// It is guaranteed that if this method returns [`Some`], then
    /// [`StrChars::next`] will also return [`Some`].
    pub fn peek(&self) -> Option<Char> {
        self.pos.get(self.s).map(Char::from)
    }

    /// Advance the iterator by one codepoint.
    ///
    /// If the iterator has reached the end, this method is a no-op.
    pub fn advance(&mut self) {
        if let Some(c) = self.pos.get(self.s) {
            self.pos.advance(c);
        }
    }
}

impl Iterator for StrChars<'_> {
    type Item = Char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.peek()?;
        self.advance();
        Some(c)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // Each UTF-8 codepoint is, at least, 1 byte. So at any given time, the
        // iterator can produce at most as many codepoints as there are bytes
        // remaining in the string.

        let rem = self
            .s
            .len()
            .checked_sub(self.pos.byte_index())
            .expect("pos points past the end of the string");

        (0, Some(rem))
    }
}

impl FusedIterator for StrChars<'_> {}
