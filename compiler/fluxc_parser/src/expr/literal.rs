use std::error::Error;

use fluxc_ast::Literal;
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, PResult, Parse, Rule};

/// Internal function to handle literal parsing failure.
fn map_parse_error<E: Error + Sized>(parse_error: E) -> CompilerError {
    panic!("{}", parse_error)
}

impl Parse for Literal {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, context: &mut Context) -> PResult<Self> {
        let node = context.new_empty(input.as_span());
        let inner = input.into_inner().next().unwrap();
        // match inner rule
        let literal = match inner.as_rule() {
            Rule::int => Literal::Int(inner.as_str().parse::<i64>().map_err(map_parse_error)?),
            Rule::float => Literal::Float(inner.as_str().parse::<f64>().map_err(map_parse_error)?),
            Rule::string => {
                Literal::String(inner.into_inner().next().unwrap().as_str().to_string())
            }
            Rule::bool => Literal::Bool(inner.as_str().parse::<bool>().map_err(map_parse_error)?),
            Rule::char => Literal::Char(
                inner
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<char>()
                    .map_err(map_parse_error)?,
            ),
            _ => unreachable!(),
        };
        Ok(node.fill(literal))
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{Literal, Node};
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{parser::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_literal_int() {
        // 123
        let mut ctx = Context::from_str("123");
        let root = ctx.create_span();
        let expected = Node { id: 0, span: root.restrict_range(0, 3), value: Literal::Int(123) };
        let result = FluxParser::parse(Rule::literal, "123").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut ctx).unwrap();
        assert_eq!(expected, result);
        // -321
        let mut ctx = Context::from_str("-321");
        let root = ctx.create_span();
        let expected = Node { id: 0, span: root.restrict_range(0, 4), value: Literal::Int(-321) };
        let result = FluxParser::parse(Rule::literal, "-321").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut ctx).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_literal_float() {
        // 123.456
        let mut context = Context::from_str("123.456");
        let root = context.create_span();
        let expected =
            Node { id: 0, span: root.restrict_range(0, 7), value: Literal::Float(123.456) };
        let result = FluxParser::parse(Rule::literal, "123.456").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
        // -123.456
        let mut context = Context::from_str("-123.456");
        let root = context.create_span();
        let expected =
            Node { id: 0, span: root.restrict_range(0, 8), value: Literal::Float(-123.456) };
        let result = FluxParser::parse(Rule::literal, "-123.456").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_literal_string() {
        // "123"
        let mut context = Context::from_str("\"123\"");
        let root = context.create_span();
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 5),
            value: Literal::String("123".to_string()),
        };
        let result = FluxParser::parse(Rule::literal, "\"123\"").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
        // "hello, world!"
        let mut context = Context::from_str("\"hello, world!\"");
        let root = context.create_span();
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 15),
            value: Literal::String("hello, world!".to_string()),
        };
        let result = FluxParser::parse(Rule::literal, "\"hello, world!\"").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
        // "🐺💖🐺" - 4 bytes per char, 2 trailing bytes, for a total of 14 bytes
        let mut context = Context::from_str("\"🐺💖🐺\"");
        let root = context.create_span();
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 14),
            value: Literal::String("🐺💖🐺".to_string()),
        };
        let result = FluxParser::parse(Rule::literal, "\"🐺💖🐺\"").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_literal_bool() {
        // true
        let mut context = Context::from_str("true");
        let root = context.create_span();
        let expected = Node { id: 0, span: root.restrict_range(0, 4), value: Literal::Bool(true) };
        let result = FluxParser::parse(Rule::literal, "true").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
        // false
        let mut context = Context::from_str("false");
        let root = context.create_span();
        let expected = Node { id: 0, span: root.restrict_range(0, 5), value: Literal::Bool(false) };
        let result = FluxParser::parse(Rule::literal, "false").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_literal_char() {
        let mut context = Context::from_str("'a'");
        let root = context.create_span();
        // 'a'
        let expected = Node { id: 0, span: root.restrict_range(0, 3), value: Literal::Char('a') };
        let result = FluxParser::parse(Rule::literal, "'a'").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
        // '🐺'
        let mut context = Context::from_str("'🐺'");
        let root = context.create_span();
        let expected =
            Node { id: 0, span: root.restrict_range(0, 6), value: Literal::Char('🐺') };
        let result = FluxParser::parse(Rule::literal, "'🐺'").unwrap().next().unwrap();
        let result = Literal::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
    }
}
