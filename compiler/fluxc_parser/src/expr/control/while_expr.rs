use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Node, While};
use fluxc_lexer::Token;

use crate::{expr::block_expr::block_expr, node, Parsers};

pub(crate) fn while_expr<'a>(
    parsers: &'a Parsers<'a>,
) -> impl Parser<Token, Node<While>, Error = Simple<Token>> + Clone + 'a {
    just(Token::KeywordWhile)
        .ignore_then(&parsers.expr)
        .then(block_expr(&parsers))
        .map(|(condition, block)| While { condition: Box::new(condition), block })
        .map_with_span(node)
}
