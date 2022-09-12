use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::Block;
use fluxc_lexer::Token;

use crate::{node, Node, Parsers};

pub(crate) fn block_expr<'a>(
    parsers: &'a Parsers<'a>,
) -> impl Parser<Token, Node<Block>, Error = Simple<Token>> + Clone + 'a {
    parsers
        .stmt
        .clone()
        .repeated()
        .delimited_by(just(Token::TokenBraceLeft), just(Token::TokenBraceRight))
        .map(|stmts| Block { stmts })
        .map_with_span(node)
}
