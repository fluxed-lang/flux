use fluxc_span::{AsSpan, Span};
use pest::RuleType;

pub trait Contains<P: PartialEq> {
	/// Test if the target contains `other`.
	fn contains(&self, other: &P) -> bool;
}

impl <P: PartialEq> Contains<P> for Option<P> {
    fn contains(&self, other: &P) -> bool {
        match self.as_ref() {
            Some(s) => s.eq(other),
            None => false,
        }
    }
}

/// Utility trait for converting Pest spans into fluxc spans.
pub trait IntoSpan {
	fn as_span(&self, src: &str) -> Span;
}

impl IntoSpan for pest::Span<'_> {
    fn as_span(&self, src: &str) -> Span {
        Span::from_str(src).restrict_range(self.start(), self.end())
    }
}

impl IntoSpan for Span {
	fn as_span(&self, _src: &str) -> Span {
		self.clone()
	}
}

impl IntoSpan for &pest::Span<'_> {
    fn as_span(&self, src: &str) -> Span {
        (*self).as_span(src)
    }
}

impl<R: RuleType> IntoSpan for pest::iterators::Pair<'_, R> {
    fn as_span(&self, src: &str) -> Span {
        pest::iterators::Pair::as_span(self).as_span(src)
    }
}

impl<R: RuleType> IntoSpan for &pest::iterators::Pair<'_, R> {
    fn as_span(&self, src: &str) -> Span {
        pest::iterators::Pair::as_span(*self).as_span(src)
    }
}
