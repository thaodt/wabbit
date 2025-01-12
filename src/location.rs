//! Source code location tracking for the Wabbit compiler
//!
//! Provides structures for tracking positions and spans in source code:
//! - Line and column numbers
//! - Source spans for error reporting
//! - Location comparison and formatting

/// Define a location in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Loc {
    pub line: usize,
    pub col: usize,
}

impl Default for Loc {
    fn default() -> Self {
        Self { line: 1, col: 0 }
    }
}

impl Loc {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    pub fn empty() -> Self {
        Self { line: 0, col: 0 }
    }
}

/// A span of source code
///
/// Spans are inclusive of both start and end.
///
/// The default span has special value where both start and end are (0, 0). It evaluates to equal
/// to any other span. This facilitates testing by making assertion on span optional.
#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Loc,
    pub end: Loc,
}

/// define how to compare two spans.
impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return true;
        }

        self.start == other.start && self.end == other.end
    }
}

impl Default for Span {
    fn default() -> Self {
        Self {
            start: Loc::empty(),
            end: Loc::empty(),
        }
    }
}

impl Span {
    pub fn new(start: Loc, end: Loc) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.start == Loc::empty() && self.end == Loc::empty()
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.start.line, self.start.col)?;

        if self.start.line == self.end.line {
            write!(f, "-{}", self.end.col)
        } else {
            write!(f, "-{}:{}", self.end.line, self.end.col)
        }
    }
}

/// minimal test this.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_span_equality() {
        assert_eq!(Span::default(), Span::default());
        // default spans are equal to everything
        assert_eq!(Span::default(), Span::new(Loc::new(1, 3), Loc::new(1, 5)));
        assert_eq!(
            Span::new(Loc::new(1, 3), Loc::new(1, 5)),
            Span::new(Loc::new(1, 3), Loc::new(1, 5))
        );
        assert_ne!(
            Span::new(Loc::new(1, 4), Loc::new(1, 5)),
            Span::new(Loc::new(1, 3), Loc::new(1, 5))
        );
    }
}
