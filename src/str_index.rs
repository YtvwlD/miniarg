use core::ops::Index;

/// A safe index into [`str`] that works over codepoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrIndex(usize);

impl StrIndex {
    /// Create a new [`StrIndex`] that points at the start of a [`str`].
    pub fn zero() -> Self {
        Self(0)
    }

    /// Get the byte index.
    ///
    /// This can be used to safely index into a [`str`].
    pub fn byte_index(self) -> usize {
        self.0
    }

    /// Increment the index with `c`.
    ///
    /// This method ensures that the index is always pointing at a [`char`]
    /// boundary.
    pub fn advance(&mut self, c: char) {
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
    type Output = str;

    fn index(&self, index: StrRange) -> &Self::Output {
        let start = index.start.byte_index();
        let end = index.end.byte_index();

        &self[start..end]
    }
}
