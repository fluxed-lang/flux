use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Node, While};
use fluxc_lexer::Token;

use crate::{
    expr::{block_expr::block_expr, expr},
    node,
};

pub(crate) fn while_expr() -> impl Parser<Token, Node<While>, Error = Simple<Token>> + Clone {
    just(Token::KeywordWhile)
        .ignore_then(expr())
        .then(block_expr())
        .map(|(condition, block)| While { condition: Box::new(condition), block })
        .map_with_span(node)
}
