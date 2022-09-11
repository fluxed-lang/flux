use chumsky::{error::Simple, primitive::choice, select, Parser};
use fluxc_ast::{Literal, Node};
use fluxc_lexer::Token;

use crate::node;

/// Parser combinator for literal strings.
pub(crate) fn literal_str() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    select! { Token::LiteralStr(x) => x }.labelled("str")
}

/// Parser combinator for literal integers.
pub(crate) fn literal_int() -> impl Parser<Token, i64, Error = Simple<Token>> + Clone {
    select! { Token::LiteralInt(x) => x}
        .try_map(|str, span| str.parse::<i64>().map_err(|e| Simple::custom(span, "")))
        .labelled("int")
}

/// Parser combinator for literal floats.
pub(crate) fn literal_float() -> impl Parser<Token, f64, Error = Simple<Token>> + Clone {
    select! { Token::LiteralFloat(x) => x }
        .try_map(|str, span| {
            str.parse::<f64>().map_err(|e| Simple::custom(span, "failed to parse float"))
        })
        .labelled("float")
}

/// Parser combinator for [Literal].
pub(crate) fn literal() -> impl Parser<Token, Node<Literal>, Error = Simple<Token>> + Clone {
    let integer = literal_int().map(Literal::Int).map_with_span(node);
    let float = literal_float().map(Literal::Float).map_with_span(node);
    let str = literal_str().map(Literal::String).map_with_span(node);
    choice((integer, float, str)).labelled("literal")
}
