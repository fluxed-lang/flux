use std::{rc::Rc, ops::Range};

/// Small struct for indexing AST nodes to a particular slice within the source
/// code. The span is byte-indexed, rather than character-indexed.
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    /// The start byte position of the span.
    pub start: usize,
    /// The end byte position of the span, inclusive.
    pub end: usize,
	/// The source text of the span.
	pub source: Rc<str>
}

impl Span {
    /// Create a new span.
    pub fn from_src<S: AsRef<str>>(src: S, start: usize, end: usize) -> Span {
        Span { source: src.as_ref().into(), start, end  }
    }

    /// Convert this span into a source code slice.
    pub fn as_slice(&self) -> &str {
        &self.source[self.start..self.end]
    }

    /// Returns true if this span includes another.
    pub const fn includes(&self, other: &Span) -> bool {
        self.start < other.start && self.end > other.end
    }

    /// Returns true if this span overlaps with another.
    pub const fn overlaps(&self, other: &Span) -> bool {
        self.start <= other.end && self.end >= other.start
    }

	pub fn restrict(&self, range: Range<usize>) -> Span {
		let start = self.start.max(range.start);
		let end = self.end.min(range.end);
		Span::from_src(self.source.clone(), start, end)
	}
}
