use std::{fmt::Debug, hash::Hash, ops::Range, path::PathBuf, rc::Rc};

/// A type alias for spans.
pub type SpanInner = Range<usize>;

/// Small struct for indexing AST nodes to a particular slice within the source
/// code. The span is byte-indexed, rather than character-indexed.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Span {
    /// The inner spanned value.
    inner: SpanInner,
    context: Rc<SpanContext>,
}

/// Associated information shared by all spans of the current source file.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SpanContext {
    /// The source text.
    pub source: String,
    /// A path to the source file.
    pub path: PathBuf,
}

impl Span {
    pub fn new(inner: Range<usize>, context: Rc<SpanContext>) -> Self {
        Span { inner, context }
    }
    /// This method returns the start index of the span.
    pub fn start(&self) -> usize {
        self.inner.start
    }
    /// This method returns the end index of the span.
    pub fn end(&self) -> usize {
        self.inner.end
    }
    /// This method returns the span as a slice.
    pub fn as_str(&self) -> &str {
        &self.context.source[self.inner.clone()]
    }
    /// This method returns the length of the span.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// This method returns true if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// This method creates a span restricted to the given range.
    pub fn restrict<R: Into<Range<usize>>>(&self, range: R) -> Self {
        Span { inner: range.into(), context: self.context.clone() }
    }
    /// This method mutably restricts the span.
    pub fn restrict_mut<R: Into<Range<usize>>>(&mut self, range: R) {
        self.inner = range.into();
    }
    /// This method returns the line number of the span.
    pub fn line(&self) -> usize {
        let mut acc = 0usize;
        for (i, char) in self.context.source.char_indices() {
            if i == self.start() {
                break;
            }
            if char == '\n' {
                acc += 1;
            }
        }
        acc + 1
    }
    /// This method returns the column number of the span.
    pub fn col(&self) -> usize {
        let mut last_newline = 0usize;
        for (i, char) in self.context.source.char_indices() {
            if i == self.start() {
                break;
            }
            if char == '\n' {
                last_newline = i;
            }
        }
        self.start() - last_newline
    }
    /// This method returns the line position of the span.
    pub fn position(&self) -> (usize, usize) {
        (self.line(), self.col())
    }
    /// This method returns the length of the span.
    pub const fn len(&self) -> usize {
        self.end - self.start
    }
}

/// Utility trait providing `into_span` for all types implementing
/// `Into<Range<usize>>`.
pub trait IntoSpan: Into<Range<usize>> {
    fn into_span(self, context: Rc<SpanContext>) -> Span {
        Span { inner: self.into(), context }
    }
}

impl<T: Into<Range<usize>>> IntoSpan for T {}

impl From<&Span> for Range<usize> {
    fn from(span: &Span) -> Self {
        span.inner.clone()
    }
}

impl ToString for Span {
    fn to_string(&self) -> String {
        self.context.source[self.inner.clone()].to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{Span, SpanContext};

    #[test]
    fn test_span_as_str() {
        let src = "hello, world!";
        let span = Span::new(0..5, Rc::new(SpanContext { source: src.into(), path: "/".into() }));

        assert_eq!("hello", span.as_str());
    }

    #[test]
    fn test_span_line_col() {
        let src = "hello\nworld!";
        let span = Span::new(7..8, Rc::new(SpanContext { source: src.into(), path: "/".into() }));

        assert_eq!(2, span.line());
        assert_eq!(2, span.col())
    }
}
