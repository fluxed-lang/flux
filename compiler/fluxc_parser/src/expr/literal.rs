use std::error::Error;

use fluxc_ast::{Literal, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

/// Internal function to handle literal parsing failure.
fn map_parse_error<E: Error + Sized>(parse_error: E) -> CompilerError {
    todo!()
}

impl Parse for Literal {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        let node = context.new_empty(input.as_span());
        let inner = input.into_inner().next().unwrap();
        // match inner rule
        let literal = match inner.as_rule() {
            Rule::int => Literal::Int(inner.as_str().parse::<i64>().map_err(map_parse_error)?),
			Rule::float => Literal::Float(inner.as_str().parse::<f64>().map_err(map_parse_error)?),
            _ => unreachable!(),
        };
        Ok(node.fill(literal))
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{Literal, Node};
    use fluxc_span::Span;
    use pest::Parser;

    use crate::{parser::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_literal_int() {
        let mut context = Context::default();
		// 123
        let expected = Node { id: 0, span: Span::new(0, 2), value: Literal::Int(123) };
        let result = FluxParser::parse(Rule::literal, "123").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(result, expected);
		// -321
		let expected = Node { id: 1, span: Span::new(0, 3), value: Literal::Int(-321) };
		let result = FluxParser::parse(Rule::literal, "-321").unwrap().next().unwrap();
		let result = Literal::parse(result, &mut context).unwrap();
		assert_eq!(result, expected);
    }

	#[test]
    fn parse_literal_float() {
        let mut context = Context::default();
		// 123.456
        let expected = Node { id: 0, span: Span::new(0, 6), value: Literal::Float(123.456) };
        let result = FluxParser::parse(Rule::literal, "123.456").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(result, expected);
		// -123.456
		let expected = Node { id: 1, span: Span::new(0, 7), value: Literal::Float(-123.456) };
		let result = FluxParser::parse(Rule::literal, "-123.456").unwrap().next().unwrap();
		let result = Literal::parse(result, &mut context).unwrap();
		assert_eq!(result, expected);
    }
}
