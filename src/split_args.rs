//! Splits a cmdline into multiple args.

pub(crate) struct SplitArgs<'a> {
    cmdline: &'a str,
    index: usize,
    quotes_begin: Option<usize>,
}

impl SplitArgs<'_> {
    pub(crate) fn new<'a>(cmdline: &'a str) -> SplitArgs<'a> {
        SplitArgs {
            cmdline,
            index: 0,
            quotes_begin: None,
        }
    }
}

impl<'a> Iterator for SplitArgs<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut index = self.index;
        if self.index >= self.cmdline.len() {
            return None
        }
        loop {
            match self.cmdline.chars().nth(index) {
                Some('\"') => {
                    if let Some(q) = self.quotes_begin {
                        let arg = &self.cmdline[q+1..index];
                        self.quotes_begin = None;
                        self.index = index + 2;
                        return Some(arg)
                    } else {
                        self.quotes_begin = Some(self.index);
                    }
                },
                Some(' ') => {
                    // Spaces only break args if we're outside of quotes.
                    if self.quotes_begin.is_none() {
                        // but spaces after quotes or spaces don't count
                        match self.cmdline.chars().nth(index-1) {
                            Some(' ') => {},
                            Some('\"') => {},
                            Some(_) => {
                                let arg = &self.cmdline[self.index..index];
                                self.index = index + 1;
                                return Some(arg)
                            },
                            None => {},
                        }
                    }
                },
                Some(_) => {}, // Ignore other characters
                None => {
                    // We're at the end.
                    let arg = &self.cmdline[self.index..index];
                    self.index = index;
                    return Some(arg)
                },
            }
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[cfg(feature = "alloc")]
    use alloc::{vec::Vec, vec};
    
    #[test]
    /// one continuous string
    fn basic() {
        let parsed: Vec<&str> = SplitArgs::new("string").collect();
        assert_eq!(parsed, vec!["string"]);
    }
    
    #[test]
    /// two strings
    fn two() {
        let parsed: Vec<&str> = SplitArgs::new("string1 string2").collect();
        assert_eq!(parsed, vec!["string1", "string2"]);
    }
    
    #[test]
    /// one string in quotes
    fn quotes()
    {
        let parsed: Vec<&str> = SplitArgs::new("\"string1 string2\"").collect();
        assert_eq!(parsed, vec!["string1 string2"]);
    }
    
    #[test]
    /// one string in quotes
    fn quotes_two()
    {
        let parsed: Vec<&str> = SplitArgs::new("\"1 2\" \"3 4\"").collect();
        assert_eq!(parsed, vec!["1 2", "3 4"]);
    }
    
    #[test]
    /// one string in quotes, two without
    fn quotes_no_quotes()
    {
        let parsed: Vec<&str> = SplitArgs::new("1 \"2 3 4\" 5").collect();
        assert_eq!(parsed, vec!["1", "2 3 4", "5"]);
    }
}
