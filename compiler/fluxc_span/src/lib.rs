use std::{fmt::Debug, ops::Range, rc::Rc};

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

    /// Restrict this span to the given range. If the span is already inside the
    /// range, it will be returned unchanged. If the span is outside the range,
    /// the produced span will have 0 length.
    pub fn restrict<S: AsSpan>(&self, other: S) -> Span {
        let span = other.as_span(&self.src);
		debug_assert_eq!(self.src, span.src);
        self.restrict_range(span.start, span.end)
    }

    /// This method restricts the span to the given range.
    pub fn restrict_range(&self, start: usize, end: usize) -> Span {
		debug_assert!(start <= end);
        Span { src: self.src.clone(), start: start.max(self.start), end: end.min(self.end) }
    }

    /// Convert this span into a source code slice.
    pub fn as_slice(&self) -> &str {
		debug_assert!(self.src.len() >= self.end);
        &self.src[self.start..self.end]
    }

    /// Returns true if this span includes another.
    pub fn includes(&self, other: &Span) -> bool {
		debug_assert_eq!(self.src, other.src);
        self.start < other.start && self.end > other.end
    }

    /// Returns true if this span overlaps with another.
    pub fn overlaps(&self, other: &Span) -> bool {
		debug_assert_eq!(self.src, other.src);
        self.start <= other.end && self.end >= other.start
    }
	/// This method returns the length of the span.
	pub const fn len(&self) -> usize {
		self.end - self.start
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

impl<R: RuleType> AsSpan for pest::iterators::Pair<'_, R> {
    fn as_span(&self, src: &str) -> Span {
        pest::iterators::Pair::as_span(&self).as_span(src)
    }
}

impl<R: RuleType> AsSpan for &pest::iterators::Pair<'_, R> {
    fn as_span(&self, src: &str) -> Span {
        pest::iterators::Pair::as_span(*self).as_span(src)
    }
}

impl AsSpan for Span {
    fn as_span(&self, _: &str) -> Span {
        self.clone()
    }
}

impl AsSpan for &Span {
    fn as_span(&self, _: &str) -> Span {
        (*self).clone()
    }
}
