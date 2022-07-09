use fluxc_span::Span;
use pest::iterators::Pair;

use crate::Rule;

/// Restrict the input span to that of the input pair.
pub fn restrict_input<'i>(input: Pair<'i, Rule>, span: &Span) -> (Pair<'i, Rule>, Span) {
    (input, span.restrict(input.as_span().start()..input.as_span().end()))
}

/// Restrict the input span to that of the second span.
pub fn restrict<'i>(a: Span, b: pest::Span<'i>) -> Span {
    a.restrict(b.start()..b.end())
}
