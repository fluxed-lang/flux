/// Small struct for indexing AST nodes to a particular slice within the source
/// code. The span is byte-indexed, rather than character-indexed.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Span {
    /// The start byte position of the span.
    start: usize,
    /// The end byte position of the span, inclusive.
    end: usize,
}

impl Span {
    /// Create a new span.
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    /// Convert this span into a source code slice.
    pub fn as_slice<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }

    /// Returns true if this span includes another.
    pub const fn includes(&self, other: &Span) -> bool {
        self.start < other.start && self.end > other.end
    }

    /// Returns true if this span overlaps with another.
    pub const fn overlaps(&self, other: &Span) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl From<pest::Span<'_>> for Span {
    fn from(span: pest::Span) -> Self {
		// -1 here to ensure that the end is inclusive
        Span::new(span.start(), span.end() - 1)
    }
}
