use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Match, MatchBranch, Node};
use fluxc_lexer::Token;

use crate::{expr::expr, node};

pub(crate) fn match_branch() -> impl Parser<Token, Node<MatchBranch>, Error = Simple<Token>> + Clone
{
    expr()
        .then_ignore(just(Token::TokenArrow))
        .then(expr())
        .map(|(pattern, value)| MatchBranch { pattern, value })
        .map_with_span(node)
}

pub(crate) fn match_expr() -> impl Parser<Token, Node<Match>, Error = Simple<Token>> + Clone {
    just(Token::KeywordMatch)
        .ignore_then(expr())
        .then(
            match_branch()
                .separated_by(just(Token::TokenComma))
                .delimited_by(just(Token::TokenBraceLeft), just(Token::TokenBracketRight)),
        )
        .map(|(expr, branches)| Match { expr: Box::new(expr), branches })
        .map_with_span(node)
}
