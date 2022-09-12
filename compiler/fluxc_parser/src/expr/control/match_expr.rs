use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Match, MatchBranch, Node};
use fluxc_lexer::Token;

use crate::{node, Parsers};

pub(crate) fn match_branch<'a>(
    parsers: &'a Parsers<'a>,
) -> impl Parser<Token, Node<MatchBranch>, Error = Simple<Token>> + Clone + 'a {
    parsers
        .expr
        .clone()
        .then_ignore(just(Token::TokenArrow))
        .then(&parsers.expr)
        .map(|(pattern, value)| MatchBranch { pattern, value })
        .map_with_span(node)
}

pub(crate) fn match_expr<'a>(
    parsers: &'a Parsers<'a>,
) -> impl Parser<Token, Node<Match>, Error = Simple<Token>> + Clone + 'a {
    just(Token::KeywordMatch)
        .ignore_then(&parsers.expr)
        .then(
            match_branch(&parsers)
                .separated_by(just(Token::TokenComma))
                .delimited_by(just(Token::TokenBraceLeft), just(Token::TokenBracketRight)),
        )
        .map(|(expr, branches)| Match { expr: Box::new(expr), branches })
        .map_with_span(node)
}
