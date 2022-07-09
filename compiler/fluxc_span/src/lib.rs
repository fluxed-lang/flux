use std::{rc::Rc, ops::Range, fmt::Debug};

use pest::RuleType;

/// Small struct for indexing AST nodes to a particular slice within the source
/// code. The span is byte-indexed, rather than character-indexed.
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    /// The start byte position of the span.
    pub start: usize,
    /// The end byte position of the span, inclusive.
    pub end: usize,
	// The source text being parsed.
	pub src: Rc<str>,
}

impl Span {
    /// Create a new span.
    pub fn from_str<S: AsRef<str>>(src: S) -> Span {
        Span { src: src.as_ref().into(), start: 0, end: src.as_ref().len() }
    }

	// Restrict this span to the given range. If the span is already inside the range,
	// it will be returned unchanged. If the span is outside the range, it will be
	pub fn restrict<S: AsSpan>(&self, other: S) -> Span {
		let span = other.as_span(&self.src);
		self.restrict_range(span.start, span.end)
	}

	/// This method restricts the span to the given range.
	pub fn restrict_range(&self, start: usize, end: usize) -> Span {
		Span { src: self.src.clone(), start: start.max(self.start), end: end.min(self.end) }
	}

    /// Convert this span into a source code slice.
    pub fn as_slice(&self) -> &str {
        &self.src[self.start..self.end]
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

/// Trait implemented by types that can be converted to `fluxc::Span` instances,
/// given some input source.
pub trait AsSpan {
	/// This method returns a new Span instance for the given input.
	fn as_span(&self, src: &str) -> Span;
}

impl AsSpan for Range<usize> {
    fn as_span(&self, src: &str) -> Span {
        Span::from_str(src).restrict_range(self.start, self.end)
    }
}

impl AsSpan for pest::Span<'_> {
	fn as_span(&self, src: &str) -> Span {
		Span::from_str(src).restrict_range(self.start(), self.end())
	}
}

impl AsSpan for &pest::Span<'_> {
	fn as_span(&self, src: &str) -> Span {
		(*self).as_span(src)
	}
}

impl <R: RuleType> AsSpan for pest::iterators::Pair<'_, R> {
    fn as_span(&self, src: &str) -> Span {
        pest::iterators::Pair::as_span(&self).as_span(src)
    }
}

impl <R: RuleType> AsSpan for &pest::iterators::Pair<'_, R> {
	fn as_span(&self, src: &str) -> Span {
		pest::iterators::Pair::as_span(*self).as_span(src)
	}
}
